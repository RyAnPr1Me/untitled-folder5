use std::fmt;

#[derive(Debug)]
pub enum PacketSnifferError {
    InterfaceNotFound(String),
    PermissionDenied,
    NetworkError(String),
    ConfigError(String),
    ExportError(String),
    InvalidFilter(String),
    IoError(std::io::Error),
}

impl fmt::Display for PacketSnifferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketSnifferError::InterfaceNotFound(interface) => {
                write!(f, "Network interface '{}' not found. Use --list-interfaces to see available interfaces.", interface)
            }
            PacketSnifferError::PermissionDenied => {
                write!(f, "Permission denied. Please run with administrator/root privileges to capture packets.")
            }
            PacketSnifferError::NetworkError(msg) => {
                write!(f, "Network error: {}. Check your network connection and interface status.", msg)
            }
            PacketSnifferError::ConfigError(msg) => {
                write!(f, "Configuration error: {}. Check your config.json file.", msg)
            }
            PacketSnifferError::ExportError(msg) => {
                write!(f, "Export error: {}. Check file permissions and disk space.", msg)
            }
            PacketSnifferError::InvalidFilter(filter) => {
                write!(f, "Invalid filter '{}'. Supported filters: tcp, udp, icmp, http, dns", filter)
            }
            PacketSnifferError::IoError(e) => {
                write!(f, "I/O error: {}. Check file permissions and disk space.", e)
            }
        }
    }
}

impl std::error::Error for PacketSnifferError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PacketSnifferError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PacketSnifferError {
    fn from(error: std::io::Error) -> Self {
        PacketSnifferError::IoError(error)
    }
}

impl From<serde_json::Error> for PacketSnifferError {
    fn from(error: serde_json::Error) -> Self {
        PacketSnifferError::ConfigError(error.to_string())
    }
}

impl From<csv::Error> for PacketSnifferError {
    fn from(error: csv::Error) -> Self {
        PacketSnifferError::ExportError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, PacketSnifferError>;

pub fn handle_error(error: &PacketSnifferError) -> ! {
    eprintln!("❌ Error: {}", error);
    
    // Provide helpful suggestions based on error type
    match error {
        PacketSnifferError::PermissionDenied => {
            eprintln!("💡 Suggestion: Try running with 'sudo' on Linux/macOS or as Administrator on Windows");
            eprintln!("   Example: sudo cargo run -- --interface eth0");
        }
        PacketSnifferError::InterfaceNotFound(_) => {
            eprintln!("💡 Suggestion: Use '--list-interfaces' to see available network interfaces");
            eprintln!("   Example: cargo run -- --list-interfaces");
        }
        PacketSnifferError::NetworkError(_) => {
            eprintln!("💡 Suggestion: Check if the network interface is up and connected");
            eprintln!("   You can use 'ip addr' (Linux) or 'ipconfig' (Windows) to check interface status");
        }
        PacketSnifferError::ConfigError(_) => {
            eprintln!("💡 Suggestion: Delete config.json to regenerate default configuration");
        }
        PacketSnifferError::ExportError(_) => {
            eprintln!("💡 Suggestion: Ensure you have write permissions and sufficient disk space");
        }
        PacketSnifferError::InvalidFilter(_) => {
            eprintln!("💡 Suggestion: Use one of these protocol filters: tcp, udp, icmp, http, dns");
        }
        PacketSnifferError::IoError(_) => {
            eprintln!("💡 Suggestion: Check file permissions and available disk space");
        }
    }
    
    std::process::exit(1);
}