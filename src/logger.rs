use log::{error, warn, info, debug};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

pub struct Logger {
    file_logger: Option<std::fs::File>,
    console_enabled: bool,
}

impl Logger {
    pub fn new(config: &crate::config::LoggingConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let file_logger = if config.enable_file {
            if let Some(ref file_path) = config.file {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)?;
                Some(file)
            } else {
                None
            }
        } else {
            None
        };

        Ok(Logger {
            file_logger,
            console_enabled: config.enable_console,
        })
    }

    pub fn log_error(&mut self, message: &str) {
        self.write_log("ERROR", message);
        if self.console_enabled {
            eprintln!("❌ {}", message);
        }
    }

    pub fn log_warn(&mut self, message: &str) {
        self.write_log("WARN", message);
        if self.console_enabled {
            println!("⚠️  {}", message);
        }
    }

    pub fn log_info(&mut self, message: &str) {
        self.write_log("INFO", message);
        if self.console_enabled {
            println!("ℹ️  {}", message);
        }
    }

    pub fn log_debug(&mut self, message: &str) {
        self.write_log("DEBUG", message);
        if self.console_enabled {
            println!("🔍 {}", message);
        }
    }

    fn write_log(&mut self, level: &str, message: &str) {
        if let Some(ref mut file) = self.file_logger {
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC");
            let log_line = format!("[{}] {} - {}\n", timestamp, level, message);
            if let Err(e) = file.write_all(log_line.as_bytes()) {
                eprintln!("Failed to write to log file: {}", e);
            }
            if let Err(e) = file.flush() {
                eprintln!("Failed to flush log file: {}", e);
            }
        }
    }

    pub fn log_packet_capture_start(&mut self, interface: &str) {
        self.log_info(&format!("Starting packet capture on interface: {}", interface));
    }

    pub fn log_packet_capture_stop(&mut self, packet_count: usize, duration_secs: u64) {
        self.log_info(&format!("Stopped packet capture. Captured {} packets in {} seconds", packet_count, duration_secs));
    }

    pub fn log_export(&mut self, format: &str, path: &str, packet_count: usize) {
        self.log_info(&format!("Exported {} packets to {} file: {}", packet_count, format, path));
    }

    pub fn log_error_with_context(&mut self, context: &str, error: &dyn std::error::Error) {
        self.log_error(&format!("{}: {}", context, error));
    }
}