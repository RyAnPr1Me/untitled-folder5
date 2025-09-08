use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
    pub export: ExportConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
    pub enable_console: bool,
    pub enable_file: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub buffer_size: usize,
    pub max_packets_per_second: usize,
    pub dashboard_refresh_rate: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportConfig {
    pub default_format: String,
    pub default_directory: String,
    pub auto_backup: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    pub colors_enabled: bool,
    pub emojis_enabled: bool,
    pub table_style: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logging: LoggingConfig {
                level: "info".to_string(),
                file: Some("packet_sniffer.log".to_string()),
                enable_console: true,
                enable_file: true,
            },
            performance: PerformanceConfig {
                buffer_size: 4096,
                max_packets_per_second: 1000,
                dashboard_refresh_rate: 1000, // milliseconds
            },
            export: ExportConfig {
                default_format: "json".to_string(),
                default_directory: "./exports".to_string(),
                auto_backup: true,
            },
            ui: UiConfig {
                colors_enabled: true,
                emojis_enabled: true,
                table_style: "modern".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save(path)?;
            Ok(config)
        }
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(path, content)?;
        Ok(())
    }
}