//! Configuration module for storing persistent app settings
//! Manages cfg.toml file to persist the user's name across sessions

use crate::i18n::{generate_name, Language};
use crate::network::{get_local_ip, extract_id_from_ip};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration structure stored in cfg.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// User's display name
    pub name: String,
    /// The language used to generate the initial name
    pub language: String,
    /// Enable/disable transfer encryption
    pub enable_transfer_encryption: bool,
    /// Application color theme
    pub app_color_theme: String,
    /// Enable/disable animations
    pub enable_animations: bool,
}

impl Default for Config {
    fn default() -> Self {
        // Generate a default name based on IP and system language
        let system_lang = Language::from_system();
        let name = match get_local_ip() {
            Ok(ip) => {
                let id = extract_id_from_ip(&ip);
                generate_name(id, system_lang)
            }
            Err(_) => {
                // Fallback to a random name if IP detection fails
                crate::i18n::generate_random_name(system_lang)
            }
        };

        Self {
            name,
            language: system_lang.code().to_string(),
            enable_transfer_encryption: true,  // 默认启用传输加密
            app_color_theme: "orange_cat".to_string(),  // 默认应用颜色主题（橘猫主题）
            enable_animations: true,  // 默认启用动画
        }
    }
}

impl Config {
    /// Load configuration from cfg.toml file
    /// If the file doesn't exist, creates a new one with default values
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create a new config with defaults and save it
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to cfg.toml file
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        let config_path = Self::config_path();
        
        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(config_path, content)?;
        Ok(())
    }

    /// Get the path to the config file
    fn config_path() -> std::path::PathBuf {
        // Use APPDATA directory on Windows, or home directory on other platforms
        #[cfg(windows)]
        let config_dir = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
        
        #[cfg(not(windows))]
        let config_dir = dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
        
        config_dir.join("catshare").join("cfg.toml")
    }

    /// Get the user's display name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set a new name and save to config
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        let _ = self.save(); // Silently fail if save fails
    }

    /// Check if the config was created with the current system language
    pub fn is_language_matching_current(&self) -> bool {
        let current_lang = Language::from_system();
        self.language == current_lang.code()
    }

    /// Get whether transfer encryption is enabled
    pub fn is_transfer_encryption_enabled(&self) -> bool {
        self.enable_transfer_encryption
    }

    /// Set whether transfer encryption is enabled
    pub fn set_transfer_encryption(&mut self, enabled: bool) {
        self.enable_transfer_encryption = enabled;
        let _ = self.save(); // Silently fail if save fails
    }

    /// Get the application color theme
    pub fn get_app_color_theme(&self) -> &str {
        &self.app_color_theme
    }

    /// Set the application color theme
    pub fn set_app_color_theme(&mut self, theme: String) {
        self.app_color_theme = theme;
        let _ = self.save(); // Silently fail if save fails
    }

    /// Get whether animations are enabled
    pub fn are_animations_enabled(&self) -> bool {
        self.enable_animations
    }

    /// Set whether animations are enabled
    pub fn set_animations(&mut self, enabled: bool) {
        self.enable_animations = enabled;
        let _ = self.save(); // Silently fail if save fails
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        assert!(!config.name.is_empty());
        assert!(!config.language.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.language, deserialized.language);
    }
}