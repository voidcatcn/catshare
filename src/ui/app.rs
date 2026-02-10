//! Main application state and UI logic for CatShare

use crate::config::Config;
use crate::i18n::{generate_name, init as init_i18n, t, Language, TranslationKey};
use crate::network::{
    connect_to_peer, format_bytes, format_duration, format_speed, get_local_ip, receive_message,
    resolve_id_to_ip, send_message, Message, NetworkInfo, TcpServer, TRANSFER_PORT, send_encrypted_message, receive_encrypted_message,
};
use crate::ui;
use crate::transfer::{
    calculate_total_size, collect_files, compress_data, decompress_data, read_file_chunks,
    FileEntry, CHUNK_SIZE,
};
use crate::ui::{
    card, danger_button, divider, drop_zone, icon_button, id_display_large, inline_alert,
    language_selector, mode_tab, primary_button, progress_bar_with_text, secondary_button,
    section_header, stat_display, status_badge_with_icon, styled_checkbox, styled_text_input,
    Colors, Sizes, TransferStatus, Particle,
};

use eframe::egui;
use egui::{CentralPanel, Color32, Context, RichText, ScrollArea, TopBottomPanel, Vec2};
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Send,
    Receive,
}

/// Application state
pub struct CatShareApp {
    // Identity
    network_info: Option<NetworkInfo>,
    language: Language,
    editing_name: bool,
    name_input: String,
    custom_name: bool,

    // Configuration
    config: Config,

    // Mode
    mode: AppMode,

    // Send mode state
    selected_files: Vec<FileEntry>,
    target_id: String,
    send_confirmed: bool,

    // Receive mode state
    save_path: PathBuf,
    incoming_request: Option<IncomingRequest>,

    // Transfer state
    status: TransferStatus,
    progress: f32,
    speed: f64,
    transferred_bytes: u64,
    total_bytes: u64,
    current_file: String,
    error_message: Option<String>,
    success_message: Option<String>,

    // Background task control
    cancel_flag: Arc<AtomicBool>,
    transfer_active: Arc<AtomicBool>,

    // Server for receiving
    server: Option<Arc<Mutex<TcpServer>>>,

    // Settings panel
    show_settings: bool,
    compression_enabled: bool,
    encryption_enabled: bool,
    app_color_theme: String,
    animations_enabled: bool,

    // UI state
    is_dragging_files: bool,

    // Particle system for background motion
    particles: Vec<Particle>,
    last_particle_time: f64,
}

/// Incoming transfer request
#[derive(Debug, Clone)]
pub struct IncomingRequest {
    pub sender_id: u8,
    pub sender_name: String,
    pub file_count: usize,
    pub total_size: u64,
    pub file_names: Vec<String>,
}

impl Default for CatShareApp {
    fn default() -> Self {
        Self::new()
    }
}

impl CatShareApp {
    pub fn new() -> Self {
        // Initialize i18n
        init_i18n();

        // Load configuration
        let config = Config::load().unwrap_or_else(|_| Config::default());

        let system_lang = Language::from_system();

        // Get network info
        let network_info = match get_local_ip() {
            Ok(ip) => {
                let id = crate::network::extract_id_from_ip(&ip);
                // Use name from config if available, otherwise generate a new one
                let name = if config.get_name().is_empty() {
                    generate_name(id, system_lang)
                } else {
                    config.get_name().to_string()
                };
                Some(NetworkInfo {
                    local_ip: ip,
                    id,
                    name,
                })
            }
            Err(_) => None,
        };

        // Default save path
        let save_path = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));

        let name_input = network_info
            .as_ref()
            .map(|n| n.name.clone())
            .unwrap_or_default();

        Self {
            network_info,
            language: system_lang,
            editing_name: false,
            name_input,
            custom_name: false,
            config: config.clone(), // Clone the config to avoid move issue
            mode: AppMode::Send,
            selected_files: Vec::new(),
            target_id: String::new(),
            send_confirmed: false,
            save_path,
            incoming_request: None,
            status: TransferStatus::Idle,
            progress: 0.0,
            speed: 0.0,
            transferred_bytes: 0,
            total_bytes: 0,
            current_file: String::new(),
            error_message: None,
            success_message: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            transfer_active: Arc::new(AtomicBool::new(false)),
            server: None,
            show_settings: false,
            compression_enabled: true,
            encryption_enabled: config.is_transfer_encryption_enabled(),
            app_color_theme: config.get_app_color_theme().to_string(),
            animations_enabled: config.are_animations_enabled(),
            is_dragging_files: false,
            particles: Vec::new(),
            last_particle_time: 0.0,
        }
    }

    /// Update name based on language (only if not custom)
    fn update_name(&mut self) {
        if !self.custom_name {
            if let Some(ref mut info) = self.network_info {
                info.name = generate_name(info.id, self.language);
                // Update the config as well
                self.config.set_name(info.name.clone());
            }
        }
    }

    /// Select files using file dialog
    fn select_files(&mut self) {
        if let Some(paths) = rfd::FileDialog::new().pick_files() {
            if let Ok(files) = collect_files(&paths) {
                self.selected_files = files;
                self.total_bytes = calculate_total_size(&self.selected_files);
                self.error_message = None;
            }
        }
    }

    /// Select folder for saving received files
    fn select_save_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.save_path = path;
        }
    }

    /// Clear selected files
    fn clear_files(&mut self) {
        self.selected_files.clear();
        self.total_bytes = 0;
    }

    /// Cancel current transfer
    fn cancel_transfer(&mut self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        self.status = TransferStatus::Cancelled;
    }

    /// Reset transfer state
    fn reset_transfer(&mut self) {
        self.status = TransferStatus::Idle;
        self.progress = 0.0;
        self.speed = 0.0;
        self.transferred_bytes = 0;
        self.current_file.clear();
        self.error_message = None;
        self.success_message = None;
        self.cancel_flag.store(false, Ordering::Relaxed);
        self.transfer_active.store(false, Ordering::Relaxed);
        self.send_confirmed = false;
        self.incoming_request = None;
    }

    /// Start sending files
    fn start_send(&mut self, ctx: &Context) {
        if self.selected_files.is_empty() || self.target_id.is_empty() {
            return;
        }

        let target_id: u8 = match self.target_id.parse() {
            Ok(id) => id,
            Err(_) => {
                self.error_message = Some(t(TranslationKey::InvalidId, self.language).to_string());
                return;
            }
        };

        let Some(ref info) = self.network_info else {
            return;
        };

        let target_ip = match resolve_id_to_ip(target_id, &info.local_ip) {
            Ok(ip) => ip,
            Err(_) => {
                self.error_message = Some(t(TranslationKey::InvalidId, self.language).to_string());
                return;
            }
        };

        let target_addr = SocketAddr::new(target_ip, TRANSFER_PORT);
        let files = self.selected_files.clone();
        let sender_id = info.id;
        let sender_name = info.name.clone();
        let total_size = self.total_bytes;
        let compress = self.compression_enabled;
        let cancel_flag = self.cancel_flag.clone();
        let transfer_active = self.transfer_active.clone();
        let ctx = ctx.clone();

        self.status = TransferStatus::Connecting;
        self.transfer_active.store(true, Ordering::Relaxed);
        self.error_message = None;

        thread::spawn(move || {
            let _result = send_files_task(
                target_addr,
                files,
                sender_id,
                sender_name,
                total_size,
                compress,
                cancel_flag,
                ctx.clone(),
            );

            transfer_active.store(false, Ordering::Relaxed);
            ctx.request_repaint();
        });
    }

    /// Start receiving server
    fn start_receive_server(&mut self, ctx: &Context) {
        let Some(ref info) = self.network_info else {
            return;
        };

        let addr = SocketAddr::new(info.local_ip, TRANSFER_PORT);

        match TcpServer::new(addr) {
            Ok(server) => {
                let server = Arc::new(Mutex::new(server));
                self.server = Some(server.clone());

                let transfer_active = self.transfer_active.clone();
                let cancel_flag = self.cancel_flag.clone();
                let ctx = ctx.clone();
                let save_path = self.save_path.clone();

                transfer_active.store(true, Ordering::Relaxed);
                self.status = TransferStatus::Connecting;

                thread::spawn(move || {
                    receive_files_task(server, save_path, cancel_flag, ctx.clone());
                    transfer_active.store(false, Ordering::Relaxed);
                    ctx.request_repaint();
                });
            }
            Err(e) => {
                self.error_message = Some(format!(
                    "{}: {}",
                    t(TranslationKey::NetworkError, self.language),
                    e
                ));
            }
        }
    }

    /// Render the header with theme background
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Logo and title - white text on dark bg
            ui.add_space(Sizes::PADDING_SM);

            // Cat emoji with white circular background for visibility
            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(36.0), egui::Sense::hover());
            if ui.is_rect_visible(icon_rect) {
                let painter = ui.painter();
                painter.circle_filled(icon_rect.center(), 18.0, Color32::WHITE);
                
                // Get theme color for the cat icon
                let icon_color = {
                    let colors: Box<dyn ui::ThemeColors> = match self.app_color_theme.as_str() {
                        "classic_blue" => Box::new(ui::ClassicBlueColors),
                        "dark_theme" => Box::new(ui::DarkThemeColors),
                        "light_theme" => Box::new(ui::LightThemeColors),
                        _ => Box::new(ui::ElectricIndigoColors), // Default to electric indigo
                    };
                    colors.primary()
                };
                
                painter.text(
                    icon_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "🐱",
                    egui::FontId::proportional(22.0),
                    icon_color,
                );
            }

            ui.add_space(Sizes::PADDING_SM);
            
            // Get theme color for text
            let text_color = {
                let colors: Box<dyn ui::ThemeColors> = match self.app_color_theme.as_str() {
                    "dark_theme" => Box::new(ui::DarkThemeColors),
                    _ => Box::new(ui::ElectricIndigoColors), // Default to light theme text
                };
                colors.text_on_primary()
            };
            
            ui.label(
                RichText::new("CatShare")
                    .size(22.0)
                    .strong()
                    .color(text_color),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(Sizes::PADDING_MD);

                // Settings button - lighter icon on dark bg
                let text_color = {
                    let colors: Box<dyn ui::ThemeColors> = match self.app_color_theme.as_str() {
                        "dark_theme" => Box::new(ui::DarkThemeColors),
                        _ => Box::new(ui::ElectricIndigoColors), // Default to light theme text
                    };
                    colors.text_on_primary()
                };
                
                let settings_btn = ui.add(
                    egui::Button::new(RichText::new("⚙").size(18.0).color(text_color))
                        .frame(false),
                );
                if settings_btn
                    .on_hover_text(t(TranslationKey::Settings, self.language))
                    .clicked()
                {
                    self.show_settings = !self.show_settings;
                }

                ui.add_space(Sizes::PADDING_SM);

                // Language selector with custom styling for dark bg
                let old_lang = self.language;
                egui::ComboBox::from_id_salt("header_lang_sel")
                    .selected_text(
                        RichText::new(format!(
                            "{} {}",
                            self.language.flag(),
                            self.language.native_name()
                        ))
                        .size(13.0)
                        .color(text_color),
                    )
                    .width(130.0)
                    .show_ui(ui, |ui| {
                        for lang in Language::all() {
                            ui.selectable_value(
                                &mut self.language,
                                *lang,
                                format!("{} {}", lang.flag(), lang.native_name()),
                            );
                        }
                    });
                if old_lang != self.language {
                    self.update_name();
                }
            });
        });
    }

    /// Render identity card
    fn render_identity(&mut self, ui: &mut egui::Ui) {
        // Get current info for display
        let (id, current_name) = match &self.network_info {
            Some(info) => (Some(info.id), Some(info.name.clone())),
            None => (None, None),
        };

        let language = self.language;
        let editing = self.editing_name;

        card(ui, |ui| {
            if let (Some(id), Some(name)) = (id, current_name.clone()) {
                ui.horizontal(|ui| {
                    // ID display (large circle)
                    id_display_large(ui, id);

                    ui.add_space(Sizes::PADDING_LG);

                    // Info column
                    ui.vertical(|ui| {
                        // Your ID label
                        ui.label(
                            RichText::new(t(TranslationKey::YourId, language))
                                .size(11.0)
                                .color(Colors::TEXT_MUTED),
                        );
                        ui.label(
                            RichText::new(format!("{}", id))
                                .size(24.0)
                                .strong()
                                .color(Colors::TEXT_HEADING),
                        );

                        ui.add_space(Sizes::PADDING_SM);

                        // Your name label
                        ui.label(
                            RichText::new(t(TranslationKey::YourName, language))
                                .size(11.0)
                                .color(Colors::TEXT_MUTED),
                        );

                        if editing {
                            // Show text input for editing name
                            ui.horizontal(|ui| {
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.name_input)
                                        .desired_width(150.0)
                                        .margin(Vec2::new(8.0, 8.0)),
                                );
                                // Save on Enter key
                                if response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    // Save the name
                                    if let Some(ref mut info) = self.network_info {
                                        if !self.name_input.trim().is_empty() {
                                            info.name = self.name_input.trim().to_string();
                                            self.custom_name = true;
                                            // Update the config as well
                                            self.config.set_name(info.name.clone());
                                        }
                                    }
                                    self.editing_name = false;
                                }
                                ui.add_space(Sizes::PADDING_SM);
                                if secondary_button(ui, t(TranslationKey::SaveName, language))
                                    .clicked()
                                {
                                    // Save the name
                                    if let Some(ref mut info) = self.network_info {
                                        if !self.name_input.trim().is_empty() {
                                            info.name = self.name_input.trim().to_string();
                                            self.custom_name = true;
                                            // Update the config as well
                                            self.config.set_name(info.name.clone());
                                        }
                                    }
                                    self.editing_name = false;
                                }
                            });
                        } else {
                            // Show name with edit button
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&name).size(16.0).color(Colors::TEXT_BODY));
                                ui.add_space(Sizes::PADDING_SM);
                                if icon_button(ui, "✏", t(TranslationKey::EditName, language))
                                    .clicked()
                                {
                                    // Start editing
                                    self.name_input = name.clone();
                                    self.editing_name = true;
                                }
                            });
                        }
                    });

                    // Copy button on the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if icon_button(ui, "📋", t(TranslationKey::CopyId, language)).clicked() {
                            ui.output_mut(|o| o.copied_text = id.to_string());
                        }
                    });
                });
            } else {
                inline_alert(
                    ui,
                    "⚠",
                    t(TranslationKey::NetworkError, language),
                    Colors::ERROR,
                );
            }
        });
    }

    /// Render mode selector tabs
    fn render_mode_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let send_clicked = mode_tab(
                ui,
                "📤",
                t(TranslationKey::Send, self.language),
                self.mode == AppMode::Send,
            )
            .clicked();

            ui.add_space(Sizes::PADDING_SM);

            let receive_clicked = mode_tab(
                ui,
                "📥",
                t(TranslationKey::Receive, self.language),
                self.mode == AppMode::Receive,
            )
            .clicked();

            if send_clicked && self.mode != AppMode::Send {
                self.mode = AppMode::Send;
                self.reset_transfer();
            }

            if receive_clicked && self.mode != AppMode::Receive {
                self.mode = AppMode::Receive;
                self.reset_transfer();
            }
        });
    }

    /// Render send mode UI
    fn render_send_mode(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        // File selection area
        card(ui, |ui| {
            section_header(ui, "📁", t(TranslationKey::SelectFiles, self.language));

            if self.selected_files.is_empty() {
                // Drop zone when no files selected
                if drop_zone(
                    ui,
                    t(TranslationKey::Browse, self.language),
                    t(TranslationKey::SelectFilesHint, self.language),
                    self.is_dragging_files,
                )
                .clicked()
                {
                    self.select_files();
                }
            } else {
                // File list
                ui.horizontal(|ui| {
                    // Stats
                    let file_count = self.selected_files.iter().filter(|f| !f.is_dir).count();
                    stat_display(
                        ui,
                        t(TranslationKey::FileCount, self.language),
                        &format!("{}", file_count),
                        Some("📄"),
                    );

                    ui.add_space(Sizes::PADDING_LG);

                    stat_display(
                        ui,
                        t(TranslationKey::TotalSize, self.language),
                        &format_bytes(self.total_bytes),
                        Some("💾"),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if secondary_button(ui, t(TranslationKey::Clear, self.language)).clicked() {
                            self.clear_files();
                        }
                        ui.add_space(Sizes::PADDING_SM);
                        if primary_button(ui, t(TranslationKey::AddMore, self.language)).clicked() {
                            self.select_files();
                        }
                    });
                });

                ui.add_space(Sizes::PADDING_MD);
                divider(ui);

                // File list with scroll
                ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    for file in &self.selected_files {
                        if !file.is_dir {
                            let icon = get_file_icon(&file.relative_path);
                            crate::ui::file_list_item(
                                ui,
                                icon,
                                &file.relative_path,
                                &format_bytes(file.size),
                                false,
                            );
                            ui.add_space(4.0);
                        }
                    }
                });
            }
        });

        ui.add_space(Sizes::PADDING_MD);

        // Target ID and send button
        card(ui, |ui| {
            section_header(ui, "🎯", t(TranslationKey::EnterId, self.language));

            ui.horizontal(|ui| {
                // ID input
                ui.label(RichText::new("ID:").size(14.0).color(Colors::TEXT_BODY));
                ui.add_space(Sizes::PADDING_SM);

                styled_text_input(
                    ui,
                    &mut self.target_id,
                    t(TranslationKey::EnterReceiverIdHint, self.language),
                    150.0,
                );

                ui.add_space(Sizes::PADDING_LG);

                // Compression toggle
                styled_checkbox(
                    ui,
                    &mut self.compression_enabled,
                    t(TranslationKey::Compress, self.language),
                );
            });
        });

        ui.add_space(Sizes::PADDING_MD);

        // Action area
        if !self.transfer_active.load(Ordering::Relaxed) {
            let can_send = !self.selected_files.is_empty() && !self.target_id.is_empty();

            ui.add_enabled_ui(can_send, |ui| {
                ui.vertical_centered(|ui| {
                    if primary_button(ui, t(TranslationKey::StartSend, self.language)).clicked() {
                        self.start_send(ctx);
                    }
                });
            });
        } else {
            self.render_transfer_progress(ui);
        }

        // Error/Success messages
        self.render_messages(ui);
    }

    /// Render receive mode UI
    fn render_receive_mode(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        // Save location card
        card(ui, |ui| {
            section_header(ui, "📂", t(TranslationKey::SaveTo, self.language));

            ui.horizontal(|ui| {
                // Path display
                let path_str = self.save_path.display().to_string();
                let display_path = if path_str.len() > 40 {
                    format!("...{}", &path_str[path_str.len() - 37..])
                } else {
                    path_str
                };

                ui.label(RichText::new("📁").size(16.0).color(Colors::ACCENT));
                ui.add_space(Sizes::PADDING_SM);
                ui.label(
                    RichText::new(display_path)
                        .size(13.0)
                        .color(Colors::TEXT_BODY),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if secondary_button(ui, t(TranslationKey::Browse, self.language)).clicked() {
                        self.select_save_folder();
                    }
                });
            });
        });

        ui.add_space(Sizes::PADDING_MD);

        // Receive status card
        card(ui, |ui| {
            if !self.transfer_active.load(Ordering::Relaxed) {
                // Waiting state
                section_header(ui, "📡", t(TranslationKey::WaitingForSender, self.language));

                ui.add_space(Sizes::PADDING_SM);

                ui.label(
                    RichText::new(t(TranslationKey::IdTooltip, self.language))
                        .size(13.0)
                        .color(Colors::TEXT_MUTED),
                );

                ui.add_space(Sizes::PADDING_LG);

                ui.vertical_centered(|ui| {
                    if primary_button(ui, t(TranslationKey::StartReceive, self.language)).clicked()
                    {
                        self.start_receive_server(ctx);
                    }
                });
            } else {
                // Active receiving
                if let Some(ref request) = self.incoming_request {
                    // Incoming request confirmation
                    section_header(ui, "📨", t(TranslationKey::IncomingRequest, self.language));

                    ui.horizontal(|ui| {
                        stat_display(
                            ui,
                            t(TranslationKey::Sender, self.language),
                            &format!("{} (ID: {})", request.sender_name, request.sender_id),
                            Some("👤"),
                        );
                    });

                    ui.add_space(Sizes::PADDING_SM);

                    ui.horizontal(|ui| {
                        stat_display(
                            ui,
                            t(TranslationKey::FileCount, self.language),
                            &format!("{}", request.file_count),
                            Some("📄"),
                        );

                        ui.add_space(Sizes::PADDING_LG);

                        stat_display(
                            ui,
                            t(TranslationKey::TotalSize, self.language),
                            &format_bytes(request.total_size),
                            Some("💾"),
                        );
                    });

                    ui.add_space(Sizes::PADDING_LG);

                    ui.horizontal(|ui| {
                        if primary_button(ui, t(TranslationKey::Accept, self.language)).clicked() {
                            // Accept transfer logic
                        }
                        ui.add_space(Sizes::PADDING_SM);
                        if danger_button(ui, t(TranslationKey::Reject, self.language)).clicked() {
                            self.reset_transfer();
                        }
                    });
                } else {
                    // Transfer in progress
                    self.render_transfer_progress(ui);
                }
            }
        });

        // Error/Success messages
        self.render_messages(ui);
    }

    /// Render transfer progress
    fn render_transfer_progress(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Status badge
            ui.horizontal(|ui| {
                status_badge_with_icon(
                    ui,
                    self.status.icon(),
                    self.status.text(self.language),
                    self.status.color(),
                );
            });

            ui.add_space(Sizes::PADDING_MD);

            // Progress bar
            let progress_text = format!("{:.1}%", self.progress * 100.0);
            progress_bar_with_text(ui, self.progress, &progress_text);

            ui.add_space(Sizes::PADDING_MD);

            // Stats row
            ui.horizontal(|ui| {
                // Speed
                stat_display(
                    ui,
                    t(TranslationKey::Speed, self.language),
                    &format_speed(self.speed),
                    Some("⚡"),
                );

                ui.add_space(Sizes::PADDING_LG);

                // Time remaining
                let remaining = if self.speed > 0.0 && self.total_bytes > self.transferred_bytes {
                    let remaining_bytes = self.total_bytes - self.transferred_bytes;
                    format_duration((remaining_bytes as f64 / self.speed) as u64)
                } else {
                    "--".to_string()
                };

                stat_display(
                    ui,
                    t(TranslationKey::TimeRemaining, self.language),
                    &remaining,
                    Some("⏱"),
                );

                ui.add_space(Sizes::PADDING_LG);

                // Transferred
                stat_display(
                    ui,
                    t(TranslationKey::Transferred, self.language),
                    &format!(
                        "{} / {}",
                        format_bytes(self.transferred_bytes),
                        format_bytes(self.total_bytes)
                    ),
                    Some("📊"),
                );
            });

            // Current file
            if !self.current_file.is_empty() {
                ui.add_space(Sizes::PADDING_SM);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📄").size(12.0).color(Colors::TEXT_MUTED));
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(&self.current_file)
                            .size(12.0)
                            .color(Colors::TEXT_MUTED),
                    );
                });
            }

            ui.add_space(Sizes::PADDING_MD);

            // Cancel button
            ui.vertical_centered(|ui| {
                if danger_button(ui, t(TranslationKey::Cancel, self.language)).clicked() {
                    self.cancel_transfer();
                }
            });
        });
    }

    /// Render error/success messages
    fn render_messages(&self, ui: &mut egui::Ui) {
        if let Some(ref error) = self.error_message {
            ui.add_space(Sizes::PADDING_MD);
            inline_alert(ui, "❌", error, Colors::ERROR);
        }

        if let Some(ref success) = self.success_message {
            ui.add_space(Sizes::PADDING_MD);
            inline_alert(ui, "✓", success, Colors::SUCCESS);
        }
    }

    /// Render settings panel
    fn render_settings(&mut self, ctx: &Context) {
        egui::Window::new(t(TranslationKey::Settings, self.language))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                egui::Frame::none()
                    .fill(Colors::SURFACE)
                    .rounding(Sizes::RADIUS_LG)
                    .inner_margin(Sizes::PADDING_LG)
                    .shadow(egui::epaint::Shadow {
                        offset: Vec2::new(0.0, 8.0),
                        blur: 24.0,
                        spread: 0.0,
                        color: Colors::SHADOW,
                    }),
            )
            .show(ctx, |ui| {
                ui.set_min_width(300.0);

                section_header(ui, "⚙", t(TranslationKey::Settings, self.language));

                ui.add_space(Sizes::PADDING_SM);

                // Language setting
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(t(TranslationKey::Language, self.language))
                            .size(14.0)
                            .color(Colors::TEXT_HEADING),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let old_lang = self.language;
                        language_selector(ui, &mut self.language);
                        if old_lang != self.language {
                            self.update_name();
                        }
                    });
                });

                ui.add_space(Sizes::PADDING_MD);

                // Compression setting
                styled_checkbox(
                    ui,
                    &mut self.compression_enabled,
                    t(TranslationKey::EnableCompression, self.language),
                );

                ui.add_space(Sizes::PADDING_SM);

                // Encryption setting
                styled_checkbox(
                    ui,
                    &mut self.encryption_enabled,
                    t(TranslationKey::EnableEncryption, self.language),
                );

                ui.add_space(Sizes::PADDING_SM);

                // Animations setting
                styled_checkbox(
                    ui,
                    &mut self.animations_enabled,
                    t(TranslationKey::EnableAnimations, self.language),
                );

                ui.add_space(Sizes::PADDING_SM);

                // Color theme selector
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(t(TranslationKey::ColorTheme, self.language))
                            .size(14.0)
                            .color(Colors::TEXT_HEADING),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Get the appropriate text color based on the current theme
                        let current_colors: Box<dyn ui::ThemeColors> = match self.app_color_theme.as_str() {
                            "classic_blue" => Box::new(ui::ClassicBlueColors),
                            "dark_theme" => Box::new(ui::DarkThemeColors),
                            "light_theme" => Box::new(ui::LightThemeColors),
                            _ => Box::new(ui::ElectricIndigoColors), // Default to electric indigo
                        };
                        
                        egui::ComboBox::from_id_salt("theme_selector")
                            .selected_text(
                                RichText::new(match self.app_color_theme.as_str() {
                                    "orange_cat" => t(TranslationKey::OrangeCatTheme, self.language),
                                    "electric_indigo" => t(TranslationKey::ElectricIndigoTheme, self.language),
                                    "classic_blue" => t(TranslationKey::ClassicBlueTheme, self.language),
                                    "dark_theme" => t(TranslationKey::DarkTheme, self.language),
                                    "light_theme" => t(TranslationKey::LightTheme, self.language),
                                    _ => t(TranslationKey::OrangeCatTheme, self.language),
                                }).color(current_colors.text_body())
                            )
                            .width(150.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.app_color_theme,
                                    "orange_cat".to_string(),
                                    RichText::new(t(TranslationKey::OrangeCatTheme, self.language))
                                        .color(current_colors.text_body()),
                                );
                                ui.selectable_value(
                                    &mut self.app_color_theme,
                                    "electric_indigo".to_string(),
                                    RichText::new(t(TranslationKey::ElectricIndigoTheme, self.language))
                                        .color(current_colors.text_body()),
                                );
                                ui.selectable_value(
                                    &mut self.app_color_theme,
                                    "classic_blue".to_string(),
                                    RichText::new(t(TranslationKey::ClassicBlueTheme, self.language))
                                        .color(current_colors.text_body()),
                                );
                                ui.selectable_value(
                                    &mut self.app_color_theme,
                                    "dark_theme".to_string(),
                                    RichText::new(t(TranslationKey::DarkTheme, self.language))
                                        .color(current_colors.text_body()),
                                );
                                ui.selectable_value(
                                    &mut self.app_color_theme,
                                    "light_theme".to_string(),
                                    RichText::new(t(TranslationKey::LightTheme, self.language))
                                        .color(current_colors.text_body()),
                                );
                            });
                    });
                });

                ui.add_space(Sizes::PADDING_LG);

                // Save settings button
                if primary_button(ui, t(TranslationKey::SaveSettings, self.language)).clicked() {
                    // Update config with new values
                    self.config.set_transfer_encryption(self.encryption_enabled);
                    self.config.set_app_color_theme(self.app_color_theme.clone());
                    self.config.set_animations(self.animations_enabled);
                }

                ui.add_space(Sizes::PADDING_SM);

                // Close button
                ui.vertical_centered(|ui| {
                    if secondary_button(ui, t(TranslationKey::Close, self.language)).clicked() {
                        self.show_settings = false;
                    }
                });
            });
    }
}

impl eframe::App for CatShareApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Request continuous repaint during transfer or if animations are enabled
        if self.transfer_active.load(Ordering::Relaxed) || self.animations_enabled {
            ctx.request_repaint_after(Duration::from_millis(16)); // ~60fps for animations
        }

        // Update theme if it has changed
        if self.app_color_theme != self.config.get_app_color_theme() {
            self.config.set_app_color_theme(self.app_color_theme.clone());
            ui::setup_custom_theme(ctx, &self.app_color_theme);
        }

        // Top panel with header - Use theme color
        let header_color = {
            let colors: Box<dyn ui::ThemeColors> = match self.app_color_theme.as_str() {
                "classic_blue" => Box::new(ui::ClassicBlueColors),
                "dark_theme" => Box::new(ui::DarkThemeColors),
                "light_theme" => Box::new(ui::LightThemeColors),
                _ => Box::new(ui::ElectricIndigoColors), // Default to electric indigo
            };
            colors.primary()
        };
        
        TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(header_color) // Use theme color
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
            .show(ctx, |ui| {
                self.render_header(ui);
            });

        // Main content - Cool Grey-Blue background
        CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Colors::BACKGROUND)
                    .inner_margin(egui::Margin::symmetric(20.0, 16.0)),
            )
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    // Identity section
                    self.render_identity(ui);

                    ui.add_space(Sizes::PADDING_LG);

                    // Mode selector
                    self.render_mode_selector(ui);

                    ui.add_space(Sizes::PADDING_LG);

                    // Mode-specific UI
                    match self.mode {
                        AppMode::Send => self.render_send_mode(ui, ctx),
                        AppMode::Receive => self.render_receive_mode(ui, ctx),
                    }
                });
            });

        // Settings panel (modal)
        if self.show_settings {
            // Dim background
            egui::Area::new(egui::Id::new("settings_backdrop"))
                .fixed_pos(egui::Pos2::ZERO)
                .show(ctx, |ui| {
                    let screen_rect = ctx.screen_rect();
                    ui.painter().rect_filled(
                        screen_rect,
                        0.0,
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120),
                    );
                });

            self.render_settings(ctx);
        }
    }
}

/// Get icon for file based on extension
fn get_file_icon(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "pdf" => "📕",
        "doc" | "docx" | "txt" | "rtf" | "md" => "📄",
        "xls" | "xlsx" | "csv" => "📊",
        "ppt" | "pptx" => "📉",
        "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp" | "bmp" => "🖼",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" => "🎬",
        "mp3" | "wav" | "flac" | "ogg" | "m4a" => "🎵",
        "zip" | "rar" | "7z" | "tar" | "gz" => "📦",
        "exe" | "msi" | "bin" => "⚙",
        "rs" | "js" | "py" | "cpp" | "c" | "h" | "html" | "css" | "json" | "toml" => "💻",
        _ => "📄",
    }
}

/// Background task for sending files
fn send_files_task(
    target_addr: SocketAddr,
    files: Vec<FileEntry>,
    sender_id: u8,
    sender_name: String,
    total_size: u64,
    compress: bool,
    cancel_flag: Arc<AtomicBool>,
    ctx: Context,
) -> Result<(), String> {
    // Connect to receiver
    let mut stream = match connect_to_peer(target_addr, Duration::from_secs(10)) {
        Ok(s) => s,
        Err(e) => return Err(format!("Connection failed: {}", e)),
    };

    // Check if encryption is enabled
    let config = crate::config::Config::load().unwrap_or_else(|_| crate::config::Config::default());
    let encryption_enabled = config.is_transfer_encryption_enabled();

    // Send connection request
    let file_names: Vec<String> = files.iter().map(|f| f.relative_path.clone()).collect();
    let request = Message::ConnectionRequest {
        sender_id,
        sender_name,
        file_count: files.len(),
        total_size,
        file_names,
    };

    if let Err(e) = send_encrypted_message(&mut stream, &request, encryption_enabled) {
        return Err(format!("Failed to send request: {}", e));
    }

    // Wait for response
    let response = match receive_encrypted_message(&mut stream, encryption_enabled) {
        Ok(msg) => msg,
        Err(e) => return Err(format!("Failed to receive response: {}", e)),
    };

    match response {
        Message::ConnectionResponse { accepted, .. } if !accepted => {
            return Err("Transfer rejected".to_string());
        }
        Message::ConnectionResponse { accepted: true, .. } => {}
        _ => return Err("Unexpected response".to_string()),
    }

    // Transfer files
    for file in files {
        if cancel_flag.load(Ordering::Relaxed) {
            let _ = send_encrypted_message(&mut stream, &Message::Cancel, encryption_enabled);
            return Err("Cancelled".to_string());
        }

        if file.is_dir {
            continue;
        }

        // Read and compress file
        let chunks = match read_file_chunks(&file.path, CHUNK_SIZE) {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to read file: {}", e)),
        };

        let chunk_count = chunks.len() as u64;

        // Send transfer start
        let start_msg = Message::TransferStart {
            file_name: file.relative_path.clone(),
            file_size: file.size,
            compressed: compress,
            chunk_count,
        };

        if let Err(e) = send_encrypted_message(&mut stream, &start_msg, encryption_enabled) {
            return Err(format!("Failed to send start: {}", e));
        }

        // Send chunks
        for (idx, chunk) in chunks.into_iter().enumerate() {
            if cancel_flag.load(Ordering::Relaxed) {
                let _ = send_encrypted_message(&mut stream, &Message::Cancel, encryption_enabled);
                return Err("Cancelled".to_string());
            }

            let data = if compress {
                compress_data(&chunk)
            } else {
                chunk
            };

            let chunk_msg = Message::TransferChunk {
                chunk_index: idx as u64,
                data,
            };

            if let Err(e) = send_encrypted_message(&mut stream, &chunk_msg, encryption_enabled) {
                return Err(format!("Failed to send chunk: {}", e));
            }
        }

        // Send complete
        let complete_msg = Message::TransferComplete { hash: 0 };
        if let Err(e) = send_encrypted_message(&mut stream, &complete_msg, encryption_enabled) {
            return Err(format!("Failed to send complete: {}", e));
        }

        // Wait for ack
        match receive_encrypted_message(&mut stream, encryption_enabled) {
            Ok(Message::Ack) => {}
            Ok(_) => return Err("Unexpected response".to_string()),
            Err(e) => return Err(format!("Failed to receive ack: {}", e)),
        }
    }

    ctx.request_repaint();
    Ok(())
}

/// Background task for receiving files
fn receive_files_task(
    server: Arc<Mutex<TcpServer>>,
    save_path: PathBuf,
    cancel_flag: Arc<AtomicBool>,
    ctx: Context,
) {
    let server_guard = server.lock().unwrap();

    // Accept connection
    let stream = match server_guard.accept(Duration::from_secs(300)) {
        Ok(Some(s)) => s,
        Ok(None) => return,
        Err(_) => return,
    };

    drop(server_guard);

    // Handle connection in separate function
    let _ = handle_incoming_connection(stream, save_path, cancel_flag, ctx);
}

fn handle_incoming_connection(
    mut stream: TcpStream,
    save_path: PathBuf,
    cancel_flag: Arc<AtomicBool>,
    ctx: Context,
) -> Result<(), String> {
    // Check if encryption is enabled
    let config = crate::config::Config::load().unwrap_or_else(|_| crate::config::Config::default());
    let encryption_enabled = config.is_transfer_encryption_enabled();

    // Receive connection request
    let request = match receive_encrypted_message(&mut stream, encryption_enabled) {
        Ok(msg) => msg,
        Err(e) => return Err(format!("Failed to receive request: {}", e)),
    };

    let (_sender_id, _sender_name, _file_count, _total_size) = match request {
        Message::ConnectionRequest {
            sender_id,
            sender_name,
            file_count,
            total_size,
            ..
        } => (sender_id, sender_name, file_count, total_size),
        _ => return Err("Unexpected message".to_string()),
    };

    // Auto-accept for now (UI would handle confirmation)
    let response = Message::ConnectionResponse {
        accepted: true,
        receiver_id: 0,
        receiver_name: "Receiver".to_string(),
    };

    if let Err(e) = send_encrypted_message(&mut stream, &response, encryption_enabled) {
        return Err(format!("Failed to send response: {}", e));
    }

    // Receive files
    loop {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Cancelled".to_string());
        }

        let msg = match receive_encrypted_message(&mut stream, encryption_enabled) {
            Ok(m) => m,
            Err(_) => break,
        };

        match msg {
            Message::TransferStart {
                file_name,
                file_size: _,
                compressed,
                chunk_count,
            } => {
                // Receive chunks
                let mut chunks = Vec::new();

                for _ in 0..chunk_count {
                    if cancel_flag.load(Ordering::Relaxed) {
                        return Err("Cancelled".to_string());
                    }

                    match receive_encrypted_message(&mut stream, encryption_enabled) {
                        Ok(Message::TransferChunk { data, .. }) => {
                            let chunk_data = if compressed {
                                decompress_data(&data).map_err(|e| e.to_string())?
                            } else {
                                data
                            };
                            chunks.push(chunk_data);
                        }
                        Ok(Message::Cancel) => return Err("Cancelled by sender".to_string()),
                        _ => return Err("Unexpected message".to_string()),
                    }
                }

                // Wait for complete message
                match receive_encrypted_message(&mut stream, encryption_enabled) {
                    Ok(Message::TransferComplete { .. }) => {}
                    _ => return Err("Expected transfer complete".to_string()),
                }

                // Write file
                let file_path = save_path.join(&file_name);
                if let Some(parent) = file_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                let mut file = std::fs::File::create(&file_path).map_err(|e| e.to_string())?;
                for chunk in chunks {
                    use std::io::Write;
                    file.write_all(&chunk).map_err(|e| e.to_string())?;
                }

                // Send ack
                let _ = send_encrypted_message(&mut stream, &Message::Ack, encryption_enabled);
            }
            Message::Cancel => return Err("Cancelled by sender".to_string()),
            _ => break,
        }

        ctx.request_repaint();
    }

    Ok(())
}
