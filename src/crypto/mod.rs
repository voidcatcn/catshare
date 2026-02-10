//! Post-Quantum Cryptography Module for CatShare
//!
//! Implements secure file transfer using:
//! - ML-KEM-768 (Kyber): Post-quantum key encapsulation
//! - ML-DSA-65 (Dilithium): Post-quantum digital signatures
//! - AES-256-GCM: Symmetric encryption
//! - SHA-3: Cryptographic hashing

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use pqcrypto_mldsa::mldsa65;
use pqcrypto_mlkem::mlkem768;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey, SharedSecret};
use pqcrypto_traits::sign::{
    DetachedSignature, PublicKey as SignPublicKey, SecretKey as SignSecretKey,
};
use sha3::{Digest, Sha3_256};
use std::error::Error;
use std::fmt;

/// Nonce size for AES-256-GCM (96 bits = 12 bytes)
pub const NONCE_SIZE: usize = 12;

/// Authentication tag size for AES-256-GCM
pub const TAG_SIZE: usize = 16;

/// Custom error type for cryptographic operations
#[derive(Debug)]
pub enum CryptoError {
    KeyGenerationFailed(String),
    EncapsulationFailed(String),
    DecapsulationFailed(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SigningFailed(String),
    VerificationFailed(String),
    InvalidKeySize,
    InvalidNonce,
    InvalidSignature,
    InvalidCiphertext,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            CryptoError::EncapsulationFailed(msg) => write!(f, "Encapsulation failed: {}", msg),
            CryptoError::DecapsulationFailed(msg) => write!(f, "Decapsulation failed: {}", msg),
            CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            CryptoError::SigningFailed(msg) => write!(f, "Signing failed: {}", msg),
            CryptoError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            CryptoError::InvalidKeySize => write!(f, "Invalid key size"),
            CryptoError::InvalidNonce => write!(f, "Invalid nonce"),
            CryptoError::InvalidSignature => write!(f, "Invalid signature"),
            CryptoError::InvalidCiphertext => write!(f, "Invalid ciphertext"),
        }
    }
}

impl Error for CryptoError {}

/// ML-KEM-768 key pair for key encapsulation
pub struct KemKeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

/// ML-DSA-65 key pair for digital signatures
pub struct SignKeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

/// Complete cryptographic identity for a peer
pub struct CryptoIdentity {
    pub kem_keypair: KemKeyPair,
    pub sign_keypair: SignKeyPair,
}

/// Encrypted message with all necessary components
#[derive(Clone)]
pub struct EncryptedMessage {
    /// ML-KEM-768 encapsulated key (ciphertext)
    pub kem_ciphertext: Vec<u8>,
    /// AES-256-GCM nonce
    pub nonce: [u8; NONCE_SIZE],
    /// AES-256-GCM encrypted data (includes auth tag)
    pub ciphertext: Vec<u8>,
    /// ML-DSA-65 signature over (kem_ciphertext || nonce || ciphertext)
    pub signature: Vec<u8>,
}

/// Session keys derived from key exchange
pub struct SessionKeys {
    /// AES-256 encryption key (32 bytes)
    pub encryption_key: [u8; 32],
}

// ============================================================================
// Key Generation
// ============================================================================

impl KemKeyPair {
    /// Generate a new ML-KEM-768 key pair
    pub fn generate() -> Result<Self, CryptoError> {
        let (pk, sk) = mlkem768::keypair();
        Ok(Self {
            public_key: pk.as_bytes().to_vec(),
            secret_key: sk.as_bytes().to_vec(),
        })
    }
}

impl SignKeyPair {
    /// Generate a new ML-DSA-65 key pair
    pub fn generate() -> Result<Self, CryptoError> {
        let (pk, sk) = mldsa65::keypair();
        Ok(Self {
            public_key: pk.as_bytes().to_vec(),
            secret_key: sk.as_bytes().to_vec(),
        })
    }
}

impl CryptoIdentity {
    /// Generate a complete cryptographic identity
    pub fn generate() -> Result<Self, CryptoError> {
        Ok(Self {
            kem_keypair: KemKeyPair::generate()?,
            sign_keypair: SignKeyPair::generate()?,
        })
    }

    /// Get the public keys for sharing with peers
    pub fn public_keys(&self) -> (Vec<u8>, Vec<u8>) {
        (
            self.kem_keypair.public_key.clone(),
            self.sign_keypair.public_key.clone(),
        )
    }
}

// ============================================================================
// Key Encapsulation (ML-KEM-768)
// ============================================================================

/// Encapsulate a shared secret using the recipient's ML-KEM-768 public key
pub fn kem_encapsulate(recipient_public_key: &[u8]) -> Result<(Vec<u8>, [u8; 32]), CryptoError> {
    let pk = mlkem768::PublicKey::from_bytes(recipient_public_key)
        .map_err(|_| CryptoError::InvalidKeySize)?;

    let (ss, ct) = mlkem768::encapsulate(&pk);

    // Derive AES-256 key from shared secret using SHA-3
    let mut hasher = Sha3_256::new();
    hasher.update(ss.as_bytes());
    let key: [u8; 32] = hasher.finalize().into();

    Ok((ct.as_bytes().to_vec(), key))
}

/// Decapsulate a shared secret using our ML-KEM-768 secret key
pub fn kem_decapsulate(kem_ciphertext: &[u8], secret_key: &[u8]) -> Result<[u8; 32], CryptoError> {
    let sk =
        mlkem768::SecretKey::from_bytes(secret_key).map_err(|_| CryptoError::InvalidKeySize)?;
    let ct = mlkem768::Ciphertext::from_bytes(kem_ciphertext)
        .map_err(|_| CryptoError::InvalidCiphertext)?;

    let ss = mlkem768::decapsulate(&ct, &sk);

    // Derive AES-256 key from shared secret using SHA-3
    let mut hasher = Sha3_256::new();
    hasher.update(ss.as_bytes());
    let key: [u8; 32] = hasher.finalize().into();

    Ok(key)
}

// ============================================================================
// Digital Signatures (ML-DSA-65)
// ============================================================================

/// Sign data using ML-DSA-65
pub fn sign(data: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let sk = mldsa65::SecretKey::from_bytes(secret_key).map_err(|_| CryptoError::InvalidKeySize)?;

    let sig = mldsa65::detached_sign(data, &sk);
    Ok(sig.as_bytes().to_vec())
}

/// Verify a ML-DSA-65 signature
pub fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, CryptoError> {
    let pk = mldsa65::PublicKey::from_bytes(public_key).map_err(|_| CryptoError::InvalidKeySize)?;
    let sig = mldsa65::DetachedSignature::from_bytes(signature)
        .map_err(|_| CryptoError::InvalidSignature)?;

    Ok(mldsa65::verify_detached_signature(&sig, data, &pk).is_ok())
}

// ============================================================================
// Symmetric Encryption (AES-256-GCM)
// ============================================================================

/// Generate a random nonce for AES-256-GCM
pub fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    getrandom::getrandom(&mut nonce).expect("Failed to generate random nonce");
    nonce
}

/// Encrypt data using AES-256-GCM
pub fn aes_encrypt(
    plaintext: &[u8],
    key: &[u8; 32],
    nonce: &[u8; NONCE_SIZE],
) -> Result<Vec<u8>, CryptoError> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let nonce_arr = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce_arr, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))
}

/// Decrypt data using AES-256-GCM
pub fn aes_decrypt(
    ciphertext: &[u8],
    key: &[u8; 32],
    nonce: &[u8; NONCE_SIZE],
) -> Result<Vec<u8>, CryptoError> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    let nonce_arr = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce_arr, ciphertext)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

// ============================================================================
// SHA-3 Hashing
// ============================================================================

/// Compute SHA3-256 hash
pub fn sha3_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute SHA3-256 hash of multiple data chunks
pub fn sha3_hash_multi(chunks: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    for chunk in chunks {
        hasher.update(chunk);
    }
    hasher.finalize().into()
}

// ============================================================================
// High-Level Secure Message API
// ============================================================================

/// Encrypt and sign a message for secure transmission
///
/// This function:
/// 1. Generates a random shared secret using ML-KEM-768
/// 2. Derives an AES-256 key from the shared secret using SHA-3
/// 3. Encrypts the plaintext using AES-256-GCM
/// 4. Signs the entire message using ML-DSA-65
pub fn secure_encrypt(
    plaintext: &[u8],
    recipient_kem_public_key: &[u8],
    sender_sign_secret_key: &[u8],
) -> Result<EncryptedMessage, CryptoError> {
    // 1. Key encapsulation (ML-KEM-768)
    let (kem_ciphertext, aes_key) = kem_encapsulate(recipient_kem_public_key)?;

    // 2. Generate nonce and encrypt (AES-256-GCM)
    let nonce = generate_nonce();
    let ciphertext = aes_encrypt(plaintext, &aes_key, &nonce)?;

    // 3. Create data to sign: kem_ciphertext || nonce || ciphertext
    let mut sign_data = Vec::new();
    sign_data.extend_from_slice(&kem_ciphertext);
    sign_data.extend_from_slice(&nonce);
    sign_data.extend_from_slice(&ciphertext);

    // 4. Sign with ML-DSA-65
    let signature = sign(&sign_data, sender_sign_secret_key)?;

    Ok(EncryptedMessage {
        kem_ciphertext,
        nonce,
        ciphertext,
        signature,
    })
}

/// Verify and decrypt a secure message
///
/// This function:
/// 1. Verifies the ML-DSA-65 signature
/// 2. Decapsulates the shared secret using ML-KEM-768
/// 3. Derives the AES-256 key from the shared secret using SHA-3
/// 4. Decrypts the ciphertext using AES-256-GCM
pub fn secure_decrypt(
    message: &EncryptedMessage,
    recipient_kem_secret_key: &[u8],
    sender_sign_public_key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    // 1. Reconstruct data that was signed
    let mut sign_data = Vec::new();
    sign_data.extend_from_slice(&message.kem_ciphertext);
    sign_data.extend_from_slice(&message.nonce);
    sign_data.extend_from_slice(&message.ciphertext);

    // 2. Verify signature (ML-DSA-65)
    if !verify(&sign_data, &message.signature, sender_sign_public_key)? {
        return Err(CryptoError::VerificationFailed(
            "Signature verification failed".to_string(),
        ));
    }

    // 3. Key decapsulation (ML-KEM-768)
    let aes_key = kem_decapsulate(&message.kem_ciphertext, recipient_kem_secret_key)?;

    // 4. Decrypt (AES-256-GCM)
    aes_decrypt(&message.ciphertext, &aes_key, &message.nonce)
}

// ============================================================================
// Serialization for Network Transfer
// ============================================================================

impl EncryptedMessage {
    /// Serialize the encrypted message for network transmission
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // KEM ciphertext length (4 bytes) + data
        bytes.extend_from_slice(&(self.kem_ciphertext.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.kem_ciphertext);

        // Nonce (fixed 12 bytes)
        bytes.extend_from_slice(&self.nonce);

        // Ciphertext length (4 bytes) + data
        bytes.extend_from_slice(&(self.ciphertext.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.ciphertext);

        // Signature length (4 bytes) + data
        bytes.extend_from_slice(&(self.signature.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.signature);

        bytes
    }

    /// Deserialize an encrypted message from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        let mut offset = 0;

        // KEM ciphertext
        if bytes.len() < offset + 4 {
            return Err(CryptoError::InvalidCiphertext);
        }
        let kem_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        if bytes.len() < offset + kem_len {
            return Err(CryptoError::InvalidCiphertext);
        }
        let kem_ciphertext = bytes[offset..offset + kem_len].to_vec();
        offset += kem_len;

        // Nonce
        if bytes.len() < offset + NONCE_SIZE {
            return Err(CryptoError::InvalidNonce);
        }
        let nonce: [u8; NONCE_SIZE] = bytes[offset..offset + NONCE_SIZE].try_into().unwrap();
        offset += NONCE_SIZE;

        // Ciphertext
        if bytes.len() < offset + 4 {
            return Err(CryptoError::InvalidCiphertext);
        }
        let ct_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        if bytes.len() < offset + ct_len {
            return Err(CryptoError::InvalidCiphertext);
        }
        let ciphertext = bytes[offset..offset + ct_len].to_vec();
        offset += ct_len;

        // Signature
        if bytes.len() < offset + 4 {
            return Err(CryptoError::InvalidSignature);
        }
        let sig_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        if bytes.len() < offset + sig_len {
            return Err(CryptoError::InvalidSignature);
        }
        let signature = bytes[offset..offset + sig_len].to_vec();

        Ok(Self {
            kem_ciphertext,
            nonce,
            ciphertext,
            signature,
        })
    }
}

// ============================================================================
// Secure Channel for Streaming Data
// ============================================================================

/// A secure channel for encrypting/decrypting streaming data
pub struct SecureChannel {
    /// AES-256 key for this channel
    aes_key: [u8; 32],
    /// Counter for nonce generation (prevents nonce reuse)
    nonce_counter: u64,
    /// Our signing secret key
    sign_secret_key: Vec<u8>,
    /// Peer's signing public key
    peer_sign_public_key: Vec<u8>,
}

impl SecureChannel {
    /// Create a new secure channel as the initiator (sender)
    pub fn new_initiator(
        recipient_kem_public_key: &[u8],
        our_sign_secret_key: &[u8],
        peer_sign_public_key: &[u8],
    ) -> Result<(Self, Vec<u8>), CryptoError> {
        let (kem_ciphertext, aes_key) = kem_encapsulate(recipient_kem_public_key)?;

        Ok((
            Self {
                aes_key,
                nonce_counter: 0,
                sign_secret_key: our_sign_secret_key.to_vec(),
                peer_sign_public_key: peer_sign_public_key.to_vec(),
            },
            kem_ciphertext,
        ))
    }

    /// Create a new secure channel as the responder (receiver)
    pub fn new_responder(
        kem_ciphertext: &[u8],
        our_kem_secret_key: &[u8],
        our_sign_secret_key: &[u8],
        peer_sign_public_key: &[u8],
    ) -> Result<Self, CryptoError> {
        let aes_key = kem_decapsulate(kem_ciphertext, our_kem_secret_key)?;

        Ok(Self {
            aes_key,
            nonce_counter: 0,
            sign_secret_key: our_sign_secret_key.to_vec(),
            peer_sign_public_key: peer_sign_public_key.to_vec(),
        })
    }

    /// Generate the next nonce (deterministic, based on counter)
    fn next_nonce(&mut self) -> [u8; NONCE_SIZE] {
        let mut nonce = [0u8; NONCE_SIZE];
        nonce[..8].copy_from_slice(&self.nonce_counter.to_le_bytes());
        self.nonce_counter += 1;
        nonce
    }

    /// Encrypt a chunk of data
    pub fn encrypt_chunk(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = self.next_nonce();
        let ciphertext = aes_encrypt(plaintext, &self.aes_key, &nonce)?;

        // Sign the chunk
        let mut sign_data = Vec::new();
        sign_data.extend_from_slice(&nonce);
        sign_data.extend_from_slice(&ciphertext);
        let signature = sign(&sign_data, &self.sign_secret_key)?;

        // Format: nonce (12) || ciphertext_len (4) || ciphertext || sig_len (4) || signature
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
        result.extend_from_slice(&ciphertext);
        result.extend_from_slice(&(signature.len() as u32).to_le_bytes());
        result.extend_from_slice(&signature);

        Ok(result)
    }

    /// Decrypt a chunk of data
    pub fn decrypt_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut offset = 0;

        // Extract nonce
        if data.len() < NONCE_SIZE {
            return Err(CryptoError::InvalidNonce);
        }
        let nonce: [u8; NONCE_SIZE] = data[..NONCE_SIZE].try_into().unwrap();
        offset += NONCE_SIZE;

        // Extract ciphertext
        if data.len() < offset + 4 {
            return Err(CryptoError::InvalidCiphertext);
        }
        let ct_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        if data.len() < offset + ct_len {
            return Err(CryptoError::InvalidCiphertext);
        }
        let ciphertext = &data[offset..offset + ct_len];
        offset += ct_len;

        // Extract signature
        if data.len() < offset + 4 {
            return Err(CryptoError::InvalidSignature);
        }
        let sig_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        if data.len() < offset + sig_len {
            return Err(CryptoError::InvalidSignature);
        }
        let signature = &data[offset..offset + sig_len];

        // Verify signature
        let mut sign_data = Vec::new();
        sign_data.extend_from_slice(&nonce);
        sign_data.extend_from_slice(ciphertext);

        if !verify(&sign_data, signature, &self.peer_sign_public_key)? {
            return Err(CryptoError::VerificationFailed(
                "Chunk signature verification failed".to_string(),
            ));
        }

        // Decrypt
        aes_decrypt(ciphertext, &self.aes_key, &nonce)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kem_encapsulation() {
        let keypair = KemKeyPair::generate().unwrap();
        let (ciphertext, key1) = kem_encapsulate(&keypair.public_key).unwrap();
        let key2 = kem_decapsulate(&ciphertext, &keypair.secret_key).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_signing() {
        let keypair = SignKeyPair::generate().unwrap();
        let message = b"Hello, Post-Quantum World!";
        let signature = sign(message, &keypair.secret_key).unwrap();
        assert!(verify(message, &signature, &keypair.public_key).unwrap());
    }

    #[test]
    fn test_aes_encryption() {
        let key = sha3_hash(b"test key");
        let nonce = generate_nonce();
        let plaintext = b"Secret message for AES-256-GCM";

        let ciphertext = aes_encrypt(plaintext, &key, &nonce).unwrap();
        let decrypted = aes_decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_secure_encrypt_decrypt() {
        let sender = CryptoIdentity::generate().unwrap();
        let recipient = CryptoIdentity::generate().unwrap();

        let plaintext = b"This is a secret message protected by post-quantum cryptography!";

        let encrypted = secure_encrypt(
            plaintext,
            &recipient.kem_keypair.public_key,
            &sender.sign_keypair.secret_key,
        )
        .unwrap();

        let decrypted = secure_decrypt(
            &encrypted,
            &recipient.kem_keypair.secret_key,
            &sender.sign_keypair.public_key,
        )
        .unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_encrypted_message_serialization() {
        let sender = CryptoIdentity::generate().unwrap();
        let recipient = CryptoIdentity::generate().unwrap();

        let plaintext = b"Test serialization";

        let encrypted = secure_encrypt(
            plaintext,
            &recipient.kem_keypair.public_key,
            &sender.sign_keypair.secret_key,
        )
        .unwrap();

        let bytes = encrypted.to_bytes();
        let deserialized = EncryptedMessage::from_bytes(&bytes).unwrap();

        let decrypted = secure_decrypt(
            &deserialized,
            &recipient.kem_keypair.secret_key,
            &sender.sign_keypair.public_key,
        )
        .unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_secure_channel() {
        let sender = CryptoIdentity::generate().unwrap();
        let recipient = CryptoIdentity::generate().unwrap();

        // Sender creates channel
        let (mut sender_channel, kem_ciphertext) = SecureChannel::new_initiator(
            &recipient.kem_keypair.public_key,
            &sender.sign_keypair.secret_key,
            &recipient.sign_keypair.public_key,
        )
        .unwrap();

        // Recipient creates channel
        let mut recipient_channel = SecureChannel::new_responder(
            &kem_ciphertext,
            &recipient.kem_keypair.secret_key,
            &recipient.sign_keypair.secret_key,
            &sender.sign_keypair.public_key,
        )
        .unwrap();

        // Test multiple chunks
        for i in 0..5 {
            let plaintext = format!("Chunk {} data", i);
            let encrypted = sender_channel.encrypt_chunk(plaintext.as_bytes()).unwrap();
            let decrypted = recipient_channel.decrypt_chunk(&encrypted).unwrap();
            assert_eq!(plaintext.as_bytes(), decrypted.as_slice());
        }
    }

    #[test]
    fn test_sha3_hash() {
        let data = b"Test data for SHA-3";
        let hash1 = sha3_hash(data);
        let hash2 = sha3_hash(data);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }
}
