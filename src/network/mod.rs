//! Network module for peer discovery and TCP connections
//! Handles IP detection, peer discovery via broadcast, and TCP file transfer

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

// Import crypto module for encryption
use crate::crypto;

/// Default port for file transfer
pub const TRANSFER_PORT: u16 = 42424;

/// Default port for discovery broadcast
pub const DISCOVERY_PORT: u16 = 42425;

/// Buffer size for network operations
pub const BUFFER_SIZE: usize = 64 * 1024; // 64KB

/// Message types for peer communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// Discovery broadcast - announce presence
    Discovery { id: u8, name: String, ip: String },
    /// Connection request from sender to receiver
    ConnectionRequest {
        sender_id: u8,
        sender_name: String,
        file_count: usize,
        total_size: u64,
        file_names: Vec<String>,
    },
    /// Response to connection request
    ConnectionResponse {
        accepted: bool,
        receiver_id: u8,
        receiver_name: String,
    },
    /// Transfer metadata
    TransferStart {
        file_name: String,
        file_size: u64,
        compressed: bool,
        chunk_count: u64,
    },
    /// Transfer chunk
    TransferChunk { chunk_index: u64, data: Vec<u8> },
    /// Transfer complete
    TransferComplete { hash: u64 },
    /// Acknowledgment
    Ack,
    /// Error
    Error { message: String },
    /// Cancel transfer
    Cancel,
    /// Encrypted message wrapper
    Encrypted { data: Vec<u8> },
    /// Key exchange request for encrypted communication
    KeyExchangeRequest { kem_public_key: Vec<u8> },
    /// Key exchange response for encrypted communication
    KeyExchangeResponse { kem_ciphertext: Vec<u8> },
}

/// Network information for this device
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub local_ip: IpAddr,
    pub id: u8,
    pub name: String,
}

impl NetworkInfo {
    /// Create new network info by detecting local IP
    pub fn new(name: String) -> Result<Self> {
        let local_ip = get_local_ip()?;
        let id = extract_id_from_ip(&local_ip);

        Ok(Self { local_ip, id, name })
    }

    /// Get the full address for file transfer
    pub fn transfer_addr(&self) -> SocketAddr {
        SocketAddr::new(self.local_ip, TRANSFER_PORT)
    }
}

/// Get the local IP address (preferring non-loopback IPv4)
pub fn get_local_ip() -> Result<IpAddr> {
    // Try to get the local IP using the local-ip-address crate
    match local_ip_address::local_ip() {
        Ok(ip) => Ok(ip),
        Err(_) => {
            // Fallback: try to find a suitable interface
            if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
                for (_, ip) in interfaces {
                    if let IpAddr::V4(ipv4) = ip {
                        if !ipv4.is_loopback() && !ipv4.is_link_local() {
                            return Ok(IpAddr::V4(ipv4));
                        }
                    }
                }
            }
            // Last resort: return localhost
            Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
        }
    }
}

/// Extract the last octet of an IP address as the ID
pub fn extract_id_from_ip(ip: &IpAddr) -> u8 {
    match ip {
        IpAddr::V4(ipv4) => ipv4.octets()[3],
        IpAddr::V6(ipv6) => {
            // Use the last byte of IPv6 address
            ipv6.octets()[15]
        }
    }
}

/// Resolve an ID to a full IP address (assumes same subnet)
pub fn resolve_id_to_ip(id: u8, local_ip: &IpAddr) -> Result<IpAddr> {
    match local_ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            Ok(IpAddr::V4(Ipv4Addr::new(
                octets[0], octets[1], octets[2], id,
            )))
        }
        IpAddr::V6(_) => Err(anyhow!("IPv6 ID resolution not supported")),
    }
}

/// TCP server for receiving files
pub struct TcpServer {
    listener: TcpListener,
    running: Arc<AtomicBool>,
}

impl TcpServer {
    /// Create a new TCP server bound to the specified address
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        Ok(Self {
            listener,
            running: Arc::new(AtomicBool::new(true)),
        })
    }

    /// Accept a connection with timeout
    pub fn accept(&self, timeout: Duration) -> Result<Option<TcpStream>> {
        let start = std::time::Instant::now();

        while start.elapsed() < timeout && self.running.load(Ordering::Relaxed) {
            match self.listener.accept() {
                Ok((stream, _addr)) => {
                    stream.set_nonblocking(false)?;
                    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(30)))?;
                    return Ok(Some(stream));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(None)
    }

    /// Stop the server
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// Get the local address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.listener.local_addr()?)
    }
}

/// Connect to a peer
pub fn connect_to_peer(addr: SocketAddr, timeout: Duration) -> Result<TcpStream> {
    let stream = TcpStream::connect_timeout(&addr, timeout)?;
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(Duration::from_secs(30)))?;
    stream.set_nodelay(true)?;
    Ok(stream)
}

/// Send a message over TCP
pub fn send_message(stream: &mut TcpStream, message: &Message) -> Result<()> {
    let data = bincode::serialize(message)?;
    let len = data.len() as u32;

    use std::io::Write;
    stream.write_all(&len.to_le_bytes())?;
    stream.write_all(&data)?;
    stream.flush()?;

    Ok(())
}

/// Send an encrypted message over TCP
pub fn send_encrypted_message(stream: &mut TcpStream, message: &Message, encryption_enabled: bool) -> Result<()> {
    if encryption_enabled {
        // For now, we'll send the message normally, but in a full implementation
        // we would encrypt the message using the crypto module
        let data = bincode::serialize(message)?;
        
        // Create an encrypted message wrapper
        let encrypted_msg = Message::Encrypted { data };
        send_message(stream, &encrypted_msg)
    } else {
        send_message(stream, message)
    }
}

/// Receive a message from TCP
pub fn receive_message(stream: &mut TcpStream) -> Result<Message> {
    use std::io::Read;

    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;

    if len > 100 * 1024 * 1024 {
        // 100MB max message size
        return Err(anyhow!("Message too large"));
    }

    let mut data = vec![0u8; len];
    stream.read_exact(&mut data)?;

    let message: Message = bincode::deserialize(&data)?;
    Ok(message)
}

/// Receive an encrypted message from TCP
pub fn receive_encrypted_message(stream: &mut TcpStream, encryption_enabled: bool) -> Result<Message> {
    let message = receive_message(stream)?;
    
    if encryption_enabled {
        // If the message is an encrypted wrapper, we need to deserialize the inner data
        match message {
            Message::Encrypted { data } => {
                // Deserialize the original message from the encrypted data
                let original_message: Message = bincode::deserialize(&data)?;
                Ok(original_message)
            }
            _ => Ok(message), // Return the message as-is if it's not wrapped
        }
    } else {
        Ok(message)
    }
}

/// UDP discovery for finding peers on the network
pub struct Discovery {
    socket: UdpSocket,
    running: Arc<AtomicBool>,
}

impl Discovery {
    /// Create a new discovery instance
    pub fn new() -> Result<Self> {
        let socket = UdpSocket::bind(("0.0.0.0", DISCOVERY_PORT))?;
        socket.set_broadcast(true)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            running: Arc::new(AtomicBool::new(true)),
        })
    }

    /// Broadcast our presence
    pub fn broadcast(&self, info: &NetworkInfo) -> Result<()> {
        let message = Message::Discovery {
            id: info.id,
            name: info.name.clone(),
            ip: info.local_ip.to_string(),
        };

        let data = bincode::serialize(&message)?;

        // Broadcast to subnet
        if let IpAddr::V4(ipv4) = info.local_ip {
            let octets = ipv4.octets();
            let broadcast_addr = SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], 255)),
                DISCOVERY_PORT,
            );
            let _ = self.socket.send_to(&data, broadcast_addr);
        }

        Ok(())
    }

    /// Listen for discovery messages
    pub fn listen(&self, timeout: Duration) -> Result<Vec<(u8, String, IpAddr)>> {
        let mut peers = Vec::new();
        let start = std::time::Instant::now();
        let mut buf = [0u8; 1024];

        while start.elapsed() < timeout && self.running.load(Ordering::Relaxed) {
            match self.socket.recv_from(&mut buf) {
                Ok((len, _addr)) => {
                    if let Ok(Message::Discovery { id, name, ip }) = bincode::deserialize::<Message>(&buf[..len]) {
                        if let Ok(parsed_ip) = ip.parse::<IpAddr>() {
                            peers.push((id, name, parsed_ip));
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(_) => break,
            }
        }

        Ok(peers)
    }

    /// Stop discovery
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format speed to human readable string
pub fn format_speed(bytes_per_sec: f64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes_per_sec >= GB {
        format!("{:.2} GB/s", bytes_per_sec / GB)
    } else if bytes_per_sec >= MB {
        format!("{:.2} MB/s", bytes_per_sec / MB)
    } else if bytes_per_sec >= KB {
        format!("{:.2} KB/s", bytes_per_sec / KB)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

/// Format duration to human readable string
pub fn format_duration(seconds: u64) -> String {
    if seconds >= 3600 {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    } else if seconds >= 60 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_id() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 114));
        assert_eq!(extract_id_from_ip(&ip), 114);
    }

    #[test]
    fn test_resolve_id() {
        let local_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let resolved = resolve_id_to_ip(114, &local_ip).unwrap();
        assert_eq!(resolved, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 114)));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
    }
}
