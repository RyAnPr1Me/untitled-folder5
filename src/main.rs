mod config;
mod logger;
mod error;

use clap::Parser;
use colored::*;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::Packet;
use prettytable::{Table, Row, Cell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use dirs;

use config::Config;
use logger::Logger;
use error::{PacketSnifferError, Result, handle_error};

#[derive(Parser)]
#[command(
    name = "packet_sniffer",
    author = "Packet Sniffer Team",
    version = "1.0.0",
    about = "Advanced Network Packet Sniffer with user-friendly interface and real-time dashboard",
    long_about = "A powerful, user-friendly network packet analyzer that captures and analyzes network traffic in real-time. Designed to make network analysis accessible to both technical experts and everyday users."
)]
struct Args {
    /// Network interface to sniff on
    #[arg(short, long)]
    interface: Option<String>,
    
    /// Filter by protocol (tcp, udp, icmp, http, dns)
    #[arg(short, long)]
    protocol: Option<String>,
    
    /// Filter by port number
    #[arg(short = 'P', long)]
    port: Option<u16>,
    
    /// Number of packets to capture (0 = unlimited)
    #[arg(short, long, default_value = "0")]
    count: usize,
    
    /// Show available network interfaces
    #[arg(short, long)]
    list_interfaces: bool,
    
    /// Enable interactive dashboard mode
    #[arg(short, long)]
    dashboard: bool,
    
    /// Export captured data to JSON file
    #[arg(long)]
    export_json: Option<String>,
    
    /// Export captured data to CSV file
    #[arg(long)]
    export_csv: Option<String>,
    
    /// Show detailed packet analysis
    #[arg(short, long)]
    verbose: bool,
    
    /// Show statistics summary every N seconds
    #[arg(long, default_value = "10")]
    stats_interval: u64,
    
    /// Configuration file path (default: ~/.config/packet_sniffer/config.json)
    #[arg(long)]
    config: Option<PathBuf>,
    
    /// Generate default configuration file and exit
    #[arg(long)]
    generate_config: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PacketInfo {
    timestamp: DateTime<Utc>,
    packet_number: usize,
    src_mac: String,
    dst_mac: String,
    src_ip: Option<String>,
    dst_ip: Option<String>,
    protocol: String,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    packet_size: usize,
    flags: Option<String>,
    payload_size: usize,
    application_protocol: Option<String>,
    description: String,
    threat_level: ThreatLevel,
    geo_info: Option<GeoInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
enum ThreatLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GeoInfo {
    country: Option<String>,
    city: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[derive(Debug, Clone)]
struct ConnectionFlow {
    src_ip: String,
    dst_ip: String,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    protocol: String,
    packet_count: usize,
    total_bytes: usize,
    first_seen: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    threat_level: ThreatLevel,
}

#[derive(Debug, Clone)]
struct BandwidthPoint {
    timestamp: DateTime<Utc>,
    bytes_per_sec: f64,
    packets_per_sec: f64,
}

#[derive(Debug, Clone)]
struct NetworkStats {
    total_packets: usize,
    total_bytes: usize,
    protocol_counts: HashMap<String, usize>,
    top_talkers: HashMap<String, usize>,
    start_time: Instant,
    bandwidth_history: Vec<BandwidthPoint>,
    connections: HashMap<String, ConnectionFlow>,
    threat_alerts: Vec<(DateTime<Utc>, String, ThreatLevel)>,
    port_activity: HashMap<u16, usize>,
    packet_sizes: Vec<usize>,
    current_connections: usize,
    peak_bandwidth: f64,
    peak_packets_per_sec: f64,
}

fn main() {
    // Initialize environment logger
    env_logger::init();
    
    let args = Args::parse();
    
    // Handle configuration generation
    if args.generate_config {
        generate_default_config(&args);
        return;
    }
    
    // Load configuration
    let config = load_configuration(&args).unwrap_or_else(|e| {
        handle_error(&e);
    });
    
    // Initialize logger
    let mut logger = Logger::new(&config.logging).unwrap_or_else(|e| {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    });
    
    logger.log_info("Starting Advanced Network Packet Sniffer v1.0.0");
    
    if args.list_interfaces {
        list_interfaces(&config, &mut logger);
        return;
    }
    
    let interface_name = match &args.interface {
        Some(name) => name.clone(),
        None => {
            let error = PacketSnifferError::InterfaceNotFound("No interface specified".to_string());
            logger.log_error_with_context("Interface selection", &error);
            handle_error(&error);
        }
    };
    
    let interface = match find_interface(&interface_name) {
        Some(iface) => iface,
        None => {
            let error = PacketSnifferError::InterfaceNotFound(interface_name);
            logger.log_error_with_context("Interface discovery", &error);
            handle_error(&error);
        }
    };
    
    // Validate protocol filter
    if let Some(ref protocol) = args.protocol {
        if !is_valid_protocol(protocol) {
            let error = PacketSnifferError::InvalidFilter(protocol.clone());
            logger.log_error_with_context("Protocol filter validation", &error);
            handle_error(&error);
        }
    }
    
    logger.log_packet_capture_start(&interface.name);
    
    let result = if args.dashboard {
        start_dashboard_mode(interface, args, config, logger)
    } else {
        start_sniffing(interface, args, config, logger)
    };
    
    if let Err(e) = result {
        handle_error(&e);
    }
}

fn generate_default_config(args: &Args) -> ! {
    let config_path = get_config_path(args);
    
    match Config::default().save(&config_path) {
        Ok(_) => {
            println!("✅ Default configuration generated at: {}", config_path.display());
            println!("💡 You can now edit this file to customize the packet sniffer behavior.");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("❌ Failed to generate configuration: {}", e);
            std::process::exit(1);
        }
    }
}

fn load_configuration(args: &Args) -> Result<Config> {
    let config_path = get_config_path(args);
    
    Config::load_or_create(config_path).map_err(|e| {
        PacketSnifferError::ConfigError(format!("Failed to load configuration: {}", e))
    })
}

fn get_config_path(args: &Args) -> PathBuf {
    if let Some(ref path) = args.config {
        path.clone()
    } else {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("packet_sniffer");
        config_dir.join("config.json")
    }
}

fn is_valid_protocol(protocol: &str) -> bool {
    matches!(protocol.to_lowercase().as_str(), "tcp" | "udp" | "icmp" | "http" | "dns")
}

fn list_interfaces(config: &Config, logger: &mut Logger) {
    println!("{}", "🌐 Available Network Interfaces:".green().bold());
    println!();
    
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Interface").style_spec("Fb"),
        Cell::new("Description").style_spec("Fb"),
        Cell::new("IP Addresses").style_spec("Fb"),
        Cell::new("Status").style_spec("Fb"),
    ]));
    
    for interface in datalink::interfaces() {
        let ips = interface.ips.iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        
        let status = if interface.is_up() { "UP".green() } else { "DOWN".red() };
        
        table.add_row(Row::new(vec![
            Cell::new(&interface.name),
            Cell::new(&interface.description),
            Cell::new(&ips),
            Cell::new(&status.to_string()),
        ]));
    }
    
    table.printstd();
    println!();
    println!("{}", "💡 Usage example:".yellow().bold());
    println!("{}", "  sudo cargo run -- --interface eth0 --dashboard".cyan());
    println!("{}", "  sudo cargo run -- --interface wlan0 --protocol http --verbose".cyan());
}

fn find_interface(name: &str) -> Option<NetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == name)
}

fn start_dashboard_mode(interface: NetworkInterface, args: Args, config: Config, mut logger: Logger) -> Result<()> {
    println!("{}", "🚀 Starting Interactive Dashboard Mode".green().bold());
    println!("{}", format!("📡 Interface: {}", interface.name).cyan());
    println!("{}", "Press Ctrl+C to stop".yellow());
    println!();
    
    let stats = Arc::new(Mutex::new(NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocol_counts: HashMap::new(),
        top_talkers: HashMap::new(),
        start_time: Instant::now(),
        bandwidth_history: Vec::new(),
        connections: HashMap::new(),
        threat_alerts: Vec::new(),
        port_activity: HashMap::new(),
        packet_sizes: Vec::new(),
        current_connections: 0,
        peak_bandwidth: 0.0,
        peak_packets_per_sec: 0.0,
    }));
    
    let captured_packets = Arc::new(Mutex::new(Vec::<PacketInfo>::new()));
    
    // Start packet capture in a separate thread
    let stats_clone = stats.clone();
    let captured_clone = captured_packets.clone();
    
    std::thread::spawn(move || {
        capture_packets_with_stats(interface, args, stats_clone, captured_clone);
    });
    
    // Display dashboard updates
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
        display_dashboard(&stats, &captured_packets);
        
        // Break on Ctrl+C (simplified version)
        // In a real implementation, you'd use signal handling
        // For now, this is an infinite loop that can only be stopped with Ctrl+C
    }
    
    // This line will never be reached due to infinite loop above
    // but is needed for compilation
    #[allow(unreachable_code)]
    Ok(())
}

fn capture_packets_with_stats(interface: NetworkInterface, args: Args, stats: std::sync::Arc<std::sync::Mutex<NetworkStats>>, captured_packets: std::sync::Arc<std::sync::Mutex<Vec<PacketInfo>>>) {
    use pnet::datalink::Channel::Ethernet;
    
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            eprintln!("Unhandled channel type");
            return;
        }
        Err(e) => {
            eprintln!("Failed to create datalink channel: {}", e);
            return;
        }
    };
    
    let mut packet_count = 0;
    
    loop {
        if args.count > 0 && packet_count >= args.count {
            break;
        }
        
        match rx.next() {
            Ok(packet) => {
                if should_capture_packet(packet, &args) {
                    let packet_info = analyze_packet_advanced(packet, packet_count + 1);
                    
                    // Update stats
                    {
                        let mut stats = stats.lock().unwrap();
                        stats.total_packets += 1;
                        stats.total_bytes += packet_info.packet_size;
                        *stats.protocol_counts.entry(packet_info.protocol.clone()).or_insert(0) += 1;
                        
                        // Track packet sizes for analysis
                        stats.packet_sizes.push(packet_info.packet_size);
                        if stats.packet_sizes.len() > 1000 {
                            stats.packet_sizes.remove(0);
                        }
                        
                        // Track port activity
                        if let Some(port) = packet_info.dst_port.or(packet_info.src_port) {
                            *stats.port_activity.entry(port).or_insert(0) += 1;
                        }
                        
                        // Track top talkers
                        if let Some(src_ip) = &packet_info.src_ip {
                            *stats.top_talkers.entry(src_ip.clone()).or_insert(0) += 1;
                        }
                        
                        // Track threat alerts
                        if packet_info.threat_level != ThreatLevel::Safe {
                            let alert_msg = format!("Suspicious {} traffic from {} to {}", 
                                packet_info.protocol,
                                packet_info.src_ip.as_ref().unwrap_or(&"unknown".to_string()),
                                packet_info.dst_ip.as_ref().unwrap_or(&"unknown".to_string())
                            );
                            stats.threat_alerts.push((packet_info.timestamp, alert_msg, packet_info.threat_level.clone()));
                            
                            // Keep only last 100 alerts
                            if stats.threat_alerts.len() > 100 {
                                stats.threat_alerts.remove(0);
                            }
                        }
                        
                        // Track connections
                        if let (Some(src_ip), Some(dst_ip)) = (&packet_info.src_ip, &packet_info.dst_ip) {
                            let connection_key = format!("{}:{}-{}:{}", 
                                src_ip, packet_info.src_port.unwrap_or(0),
                                dst_ip, packet_info.dst_port.unwrap_or(0)
                            );
                            
                            let connection = stats.connections.entry(connection_key.clone()).or_insert(ConnectionFlow {
                                src_ip: src_ip.clone(),
                                dst_ip: dst_ip.clone(),
                                src_port: packet_info.src_port,
                                dst_port: packet_info.dst_port,
                                protocol: packet_info.protocol.clone(),
                                packet_count: 0,
                                total_bytes: 0,
                                first_seen: packet_info.timestamp,
                                last_seen: packet_info.timestamp,
                                threat_level: packet_info.threat_level.clone(),
                            });
                            
                            connection.packet_count += 1;
                            connection.total_bytes += packet_info.packet_size;
                            connection.last_seen = packet_info.timestamp;
                            
                            // Update threat level if higher
                            if packet_info.threat_level > connection.threat_level {
                                connection.threat_level = packet_info.threat_level.clone();
                            }
                        }
                        
                        // Calculate bandwidth stats every few seconds
                        let elapsed = stats.start_time.elapsed().as_secs();
                        if elapsed > 0 && stats.total_packets % 100 == 0 {
                            let bytes_per_sec = stats.total_bytes as f64 / elapsed as f64;
                            let packets_per_sec = stats.total_packets as f64 / elapsed as f64;
                            
                            stats.bandwidth_history.push(BandwidthPoint {
                                timestamp: packet_info.timestamp,
                                bytes_per_sec,
                                packets_per_sec,
                            });
                            
                            // Update peaks
                            if bytes_per_sec > stats.peak_bandwidth {
                                stats.peak_bandwidth = bytes_per_sec;
                            }
                            if packets_per_sec > stats.peak_packets_per_sec {
                                stats.peak_packets_per_sec = packets_per_sec;
                            }
                            
                            // Keep only last 100 bandwidth points
                            if stats.bandwidth_history.len() > 100 {
                                stats.bandwidth_history.remove(0);
                            }
                        }
                        
                        // Update current connections count
                        stats.current_connections = stats.connections.len();
                    }
                    
                    // Store packet info
                    {
                        let mut packets = captured_packets.lock().unwrap();
                        packets.push(packet_info);
                        
                        // Keep only last 1000 packets to avoid memory issues
                        if packets.len() > 1000 {
                            packets.remove(0);
                        }
                    }
                    
                    packet_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Failed to read packet: {}", e);
                break;
            }
        }
    }
}

fn display_dashboard(stats: &std::sync::Arc<std::sync::Mutex<NetworkStats>>, captured_packets: &std::sync::Arc<std::sync::Mutex<Vec<PacketInfo>>>) {
    let stats = stats.lock().unwrap();
    let packets = captured_packets.lock().unwrap();
    
    // Clear screen and display header
    println!("{}", "\x1B[2J\x1B[1;1H");
    println!("{}", "🚀 ADVANCED NETWORK TRAFFIC DASHBOARD".green().bold());
    println!("{}", "═".repeat(100).blue());
    
    let duration = stats.start_time.elapsed().as_secs();
    let packets_per_sec = if duration > 0 { stats.total_packets as f64 / duration as f64 } else { 0.0 };
    let bytes_per_sec = if duration > 0 { stats.total_bytes as f64 / duration as f64 } else { 0.0 };
    
    // Main statistics overview
    println!("⏱️  {} {} {} {} {} {} {} {}", 
             "Duration:".cyan(), format!("{}s", duration).yellow().bold(),
             "| 📦 Packets:".cyan(), format!("{} ({:.1}/s)", stats.total_packets, packets_per_sec).yellow().bold(),
             "| 📊 Data:".cyan(), format!("{} ({:.1}/s)", format_bytes(stats.total_bytes), bytes_per_sec).yellow().bold(),
             "| 🔗 Connections:".cyan(), format!("{}", stats.current_connections).yellow().bold()
    );
    
    // Performance metrics
    println!("⚡ {} {} {} {}", 
             "Peak Bandwidth:".cyan(), format!("{}/s", format_bytes(stats.peak_bandwidth as usize)).red().bold(),
             "| Peak Packets:".cyan(), format!("{:.1}/s", stats.peak_packets_per_sec).red().bold()
    );
    println!();
    
    // Real-time bandwidth graph (ASCII art)
    display_bandwidth_graph(&stats.bandwidth_history);
    
    // Security threat indicators
    display_threat_dashboard(&stats.threat_alerts, &packets);
    
    // Split dashboard into columns
    println!("{}", "┌─────────────────────────────────────────────────┬─────────────────────────────────────────────────┐".blue());
    print!("{}", "│".blue());
    print!("{:^49}", "🔗 PROTOCOL ANALYSIS".yellow().bold());
    print!("{}", "│".blue());
    print!("{:^49}", "🌍 TOP CONNECTIONS".yellow().bold());
    println!("{}", "│".blue());
    println!("{}", "├─────────────────────────────────────────────────┼─────────────────────────────────────────────────┤".blue());
    
    // Display protocol stats and top connections side by side
    display_protocol_and_connections(&stats);
    
    println!("{}", "└─────────────────────────────────────────────────┴─────────────────────────────────────────────────┘".blue());
    
    // Port activity analysis
    display_port_activity(&stats.port_activity);
    
    // Packet size distribution
    display_packet_size_analysis(&stats.packet_sizes);
    
    // Geographic distribution
    display_geographic_analysis(&packets);
    
    // Recent activity stream
    display_recent_activity(&packets);
    
    // Footer with controls
    println!("\n{}", "═".repeat(100).blue());
    println!("{}", "💡 CONTROLS: [Ctrl+C] Exit | [Space] Pause | [F] Filter | [E] Export | [H] Help".cyan());
    println!("{}", format!("📡 Last Updated: {}", Utc::now().format("%H:%M:%S UTC")).bright_black());
}

fn format_bytes(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

fn display_bandwidth_graph(bandwidth_history: &Vec<BandwidthPoint>) {
    println!("{}", "📈 REAL-TIME BANDWIDTH GRAPH".yellow().bold());
    
    if bandwidth_history.is_empty() {
        println!("   {}", "No data available yet...".bright_black());
        println!();
        return;
    }
    
    let max_bytes = bandwidth_history.iter()
        .map(|p| p.bytes_per_sec)
        .fold(0.0, f64::max)
        .max(1.0); // Prevent division by zero
    
    println!("   {} {}/s", "Peak:".cyan(), format_bytes(max_bytes as usize).red().bold());
    
    // ASCII graph
    for point in bandwidth_history.iter().rev().take(20).rev() {
        let bar_length = ((point.bytes_per_sec / max_bytes) * 40.0) as usize;
        let bar = "█".repeat(bar_length);
        let time = point.timestamp.format("%H:%M:%S").to_string();
        println!("   {} │{:<40}│ {}", 
                 time.bright_black(), 
                 bar.green(), 
                 format_bytes(point.bytes_per_sec as usize).cyan());
    }
    println!();
}

fn display_threat_dashboard(threat_alerts: &Vec<(DateTime<Utc>, String, ThreatLevel)>, packets: &Vec<PacketInfo>) {
    let threat_counts = packets.iter().fold([0; 5], |mut acc, packet| {
        match packet.threat_level {
            ThreatLevel::Safe => acc[0] += 1,
            ThreatLevel::Low => acc[1] += 1,
            ThreatLevel::Medium => acc[2] += 1,
            ThreatLevel::High => acc[3] += 1,
            ThreatLevel::Critical => acc[4] += 1,
        }
        acc
    });
    
    let total_threats = threat_counts[1] + threat_counts[2] + threat_counts[3] + threat_counts[4];
    
    println!("{} {} {}", 
             "🛡️  SECURITY STATUS:".yellow().bold(),
             if total_threats == 0 { "✅ SECURE".green().bold() } else { "⚠️  THREATS DETECTED".red().bold() },
             format!("({} alerts)", threat_alerts.len()).bright_black()
    );
    
    // Threat level bars
    let threat_bar = format!("Safe:{} Low:{} Med:{} High:{} Crit:{}", 
                            threat_counts[0], threat_counts[1], threat_counts[2], threat_counts[3], threat_counts[4]);
    println!("   {}", threat_bar.cyan());
    
    // Recent threat alerts
    if !threat_alerts.is_empty() {
        println!("   {} Recent Alerts:", "🚨".red());
        for (timestamp, message, level) in threat_alerts.iter().rev().take(3) {
            let level_icon = match level {
                ThreatLevel::Low => "🟡",
                ThreatLevel::Medium => "🟠", 
                ThreatLevel::High => "🔴",
                ThreatLevel::Critical => "💀",
                _ => "⚪",
            };
            println!("   {} {} {}", 
                     level_icon, 
                     timestamp.format("%H:%M:%S").to_string().bright_black(),
                     message.yellow());
        }
    }
    println!();
}

fn display_protocol_and_connections(stats: &NetworkStats) {
    // Prepare protocol data
    let mut protocols: Vec<_> = stats.protocol_counts.iter().collect();
    protocols.sort_by(|a, b| b.1.cmp(a.1));
    
    // Prepare connection data
    let mut connections: Vec<_> = stats.connections.iter().collect();
    connections.sort_by(|a, b| b.1.packet_count.cmp(&a.1.packet_count));
    
    let max_rows = std::cmp::max(protocols.len(), connections.len().min(8));
    
    for i in 0..max_rows.max(5) {
        print!("{}", "│".blue());
        
        // Protocol column
        if i < protocols.len() {
            let (protocol, count) = protocols[i];
            let percentage = (*count as f64 / stats.total_packets as f64) * 100.0;
            print!(" {:<12} {:>8} {:>6.1}%{:>18}", 
                   protocol.green(), 
                   count.to_string().yellow(), 
                   percentage,
                   "");
        } else {
            print!("{:49}", "");
        }
        
        print!("{}", "│".blue());
        
        // Connection column
        if i < connections.len() && i < 8 {
            let (_, connection) = connections[i];
            let threat_icon = match connection.threat_level {
                ThreatLevel::Safe => "✅",
                ThreatLevel::Low => "🟡",
                ThreatLevel::Medium => "🟠",
                ThreatLevel::High => "🔴", 
                ThreatLevel::Critical => "💀",
            };
            
            let conn_display = format!("{}→{}", 
                connection.src_ip.split('.').last().unwrap_or("?"),
                connection.dst_ip.split('.').last().unwrap_or("?"));
            
            print!(" {} {:<15} {:>8} {:>8}", 
                   threat_icon,
                   conn_display.blue(),
                   connection.packet_count.to_string().yellow(),
                   format_bytes(connection.total_bytes).cyan());
        } else {
            print!("{:49}", "");
        }
        
        println!("{}", "│".blue());
    }
}

fn display_port_activity(port_activity: &HashMap<u16, usize>) {
    println!("{}", "🚪 TOP PORT ACTIVITY".yellow().bold());
    
    if port_activity.is_empty() {
        println!("   {}", "No port activity recorded yet...".bright_black());
        println!();
        return;
    }
    
    let mut ports: Vec<_> = port_activity.iter().collect();
    ports.sort_by(|a, b| b.1.cmp(a.1));
    
    print!("   ");
    for (port, count) in ports.iter().take(10) {
        let port_color = match **port {
            80 | 443 => "green",
            22 | 23 => "yellow", 
            53 => "blue",
            _ if **port > 1024 => "cyan",
            _ => "red",
        };
        
        print!("{}:{} ", 
               match port_color {
                   "green" => format!("{}", port).green(),
                   "yellow" => format!("{}", port).yellow(),
                   "blue" => format!("{}", port).blue(),
                   "cyan" => format!("{}", port).cyan(),
                   _ => format!("{}", port).red(),
               },
               count.to_string().bright_black());
    }
    println!("\n");
}

fn display_packet_size_analysis(packet_sizes: &Vec<usize>) {
    println!("{}", "📏 PACKET SIZE DISTRIBUTION".yellow().bold());
    
    if packet_sizes.is_empty() {
        println!("   {}", "No packet size data available...".bright_black());
        println!();
        return;
    }
    
    let avg_size = packet_sizes.iter().sum::<usize>() as f64 / packet_sizes.len() as f64;
    let min_size = packet_sizes.iter().min().unwrap_or(&0);
    let max_size = packet_sizes.iter().max().unwrap_or(&0);
    
    // Size categories
    let small = packet_sizes.iter().filter(|&&s| s < 100).count();
    let medium = packet_sizes.iter().filter(|&&s| s >= 100 && s < 500).count();
    let large = packet_sizes.iter().filter(|&&s| s >= 500 && s < 1500).count();
    let jumbo = packet_sizes.iter().filter(|&&s| s >= 1500).count();
    
    println!("   {} {} {} {} {} {} {} {}", 
             "Avg:".cyan(), format!("{}B", avg_size as usize).yellow(),
             "Range:".cyan(), format!("{}-{}B", min_size, max_size).yellow(),
             "Small:".cyan(), small.to_string().green(),
             "Large:".cyan(), (large + jumbo).to_string().red());
    
    // Simple histogram
    let total = packet_sizes.len();
    let small_bar = "█".repeat((small * 30 / total.max(1)).min(30));
    let medium_bar = "█".repeat((medium * 30 / total.max(1)).min(30));
    let large_bar = "█".repeat(((large + jumbo) * 30 / total.max(1)).min(30));
    
    println!("   <100B  │{:<30}│ {}%", small_bar.green(), (small * 100 / total.max(1)));
    println!("   100-500│{:<30}│ {}%", medium_bar.yellow(), (medium * 100 / total.max(1)));
    println!("   >500B  │{:<30}│ {}%", large_bar.red(), ((large + jumbo) * 100 / total.max(1)));
    println!();
}

fn display_geographic_analysis(packets: &Vec<PacketInfo>) {
    println!("{}", "🌍 GEOGRAPHIC DISTRIBUTION".yellow().bold());
    
    let mut country_counts = HashMap::new();
    for packet in packets.iter().rev().take(500) {
        if let Some(ref geo) = packet.geo_info {
            if let Some(ref country) = geo.country {
                *country_counts.entry(country.clone()).or_insert(0) += 1;
            }
        }
    }
    
    if country_counts.is_empty() {
        println!("   {}", "No geographic data available...".bright_black());
        println!();
        return;
    }
    
    let mut countries: Vec<_> = country_counts.iter().collect();
    countries.sort_by(|a, b| b.1.cmp(a.1));
    
    print!("   ");
    for (country, count) in countries.iter().take(6) {
        let flag = match country.as_str() {
            "United States" => "🇺🇸",
            "United Kingdom" => "🇬🇧", 
            "Australia" => "🇦🇺",
            "Germany" => "🇩🇪",
            "France" => "🇫🇷",
            "Local Network" => "🏠",
            _ => "🌐",
        };
        
        print!("{} {}: {} ", flag, country.cyan(), count.to_string().yellow());
    }
    println!("\n");
}

fn display_recent_activity(packets: &Vec<PacketInfo>) {
    println!("{}", "📋 LIVE ACTIVITY STREAM".yellow().bold());
    
    if packets.is_empty() {
        println!("   {}", "Waiting for network activity...".bright_black());
        println!();
        return;
    }
    
    for packet in packets.iter().rev().take(8) {
        let timestamp = packet.timestamp.format("%H:%M:%S%.1f").to_string();
        let threat_icon = match packet.threat_level {
            ThreatLevel::Safe => "✅",
            ThreatLevel::Low => "🟡",
            ThreatLevel::Medium => "🟠", 
            ThreatLevel::High => "🔴",
            ThreatLevel::Critical => "💀",
        };
        
        let app_proto = packet.application_protocol.as_ref()
            .map(|s| format!(" ({})", s))
            .unwrap_or_default();
            
        let geo_info = packet.geo_info.as_ref()
            .and_then(|g| g.country.as_ref())
            .map(|c| if c == "Local Network" { "🏠" } else { "🌐" })
            .unwrap_or("");
        
        println!("   {} {} {} {} {} → {} {} {}{}", 
                 threat_icon,
                 timestamp.bright_black(),
                 packet.protocol.green().bold(),
                 app_proto.yellow(),
                 packet.src_ip.as_ref().unwrap_or(&"?".to_string()).blue(),
                 packet.dst_ip.as_ref().unwrap_or(&"?".to_string()).blue(),
                 geo_info,
                 format_bytes(packet.packet_size).cyan(),
                 if packet.packet_size > 1000 { " 📈" } else { "" });
    }
    println!();
}

fn analyze_packet_advanced(packet: &[u8], packet_num: usize) -> PacketInfo {
    let timestamp = Utc::now();
    let packet_size = packet.len();
    
    let mut packet_info = PacketInfo {
        timestamp,
        packet_number: packet_num,
        src_mac: String::new(),
        dst_mac: String::new(),
        src_ip: None,
        dst_ip: None,
        protocol: "Unknown".to_string(),
        src_port: None,
        dst_port: None,
        packet_size,
        flags: None,
        payload_size: 0,
        application_protocol: None,
        description: "Unknown packet".to_string(),
        threat_level: ThreatLevel::Safe,
        geo_info: None,
    };
    
    if let Some(ethernet_packet) = EthernetPacket::new(packet) {
        packet_info.src_mac = ethernet_packet.get_source().to_string();
        packet_info.dst_mac = ethernet_packet.get_destination().to_string();
        
        match ethernet_packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    packet_info.src_ip = Some(ipv4_packet.get_source().to_string());
                    packet_info.dst_ip = Some(ipv4_packet.get_destination().to_string());
                    
                    match ipv4_packet.get_next_level_protocol() {
                        pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                            packet_info.protocol = "TCP".to_string();
                            if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                packet_info.src_port = Some(tcp_packet.get_source());
                                packet_info.dst_port = Some(tcp_packet.get_destination());
                                packet_info.payload_size = tcp_packet.payload().len();
                                
                                let flags = tcp_packet.get_flags();
                                let mut flag_str = String::new();
                                if flags & 0x01 != 0 { flag_str.push_str("FIN "); }
                                if flags & 0x02 != 0 { flag_str.push_str("SYN "); }
                                if flags & 0x04 != 0 { flag_str.push_str("RST "); }
                                if flags & 0x08 != 0 { flag_str.push_str("PSH "); }
                                if flags & 0x10 != 0 { flag_str.push_str("ACK "); }
                                if flags & 0x20 != 0 { flag_str.push_str("URG "); }
                                packet_info.flags = Some(flag_str.trim().to_string());
                                
                                // Detect application protocols
                                packet_info.application_protocol = detect_application_protocol(tcp_packet.get_destination(), tcp_packet.payload());
                                packet_info.description = format_packet_description(&packet_info);
                            }
                        }
                        pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                            packet_info.protocol = "UDP".to_string();
                            if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                packet_info.src_port = Some(udp_packet.get_source());
                                packet_info.dst_port = Some(udp_packet.get_destination());
                                packet_info.payload_size = udp_packet.payload().len();
                                
                                packet_info.application_protocol = detect_application_protocol(udp_packet.get_destination(), udp_packet.payload());
                                packet_info.description = format_packet_description(&packet_info);
                            }
                        }
                        pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                            packet_info.protocol = "ICMP".to_string();
                            if let Some(_icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                                packet_info.description = "ICMP ping/echo message".to_string();
                            }
                        }
                        _ => {
                            packet_info.protocol = format!("IPv4-{:?}", ipv4_packet.get_next_level_protocol());
                        }
                    }
                }
            }
            EtherTypes::Ipv6 => {
                packet_info.protocol = "IPv6".to_string();
                packet_info.description = "IPv6 packet (parsing not fully implemented)".to_string();
            }
            _ => {
                packet_info.protocol = format!("{:?}", ethernet_packet.get_ethertype());
            }
        }
    }
    
    // Add threat detection
    packet_info.threat_level = detect_threat_level(&packet_info);
    
    // Add geographical information (simplified for demo)
    if let Some(ref dst_ip) = packet_info.dst_ip {
        packet_info.geo_info = get_geo_info(dst_ip);
    }
    
    packet_info
}

fn detect_application_protocol(port: u16, payload: &[u8]) -> Option<String> {
    match port {
        80 | 8080 => {
            // Check for HTTP
            if payload.len() > 0 {
                let payload_str = String::from_utf8_lossy(&payload[..std::cmp::min(100, payload.len())]);
                if payload_str.starts_with("GET") || payload_str.starts_with("POST") || 
                   payload_str.starts_with("HTTP") || payload_str.contains("Host:") {
                    return Some("HTTP".to_string());
                }
            }
            Some("Web Traffic".to_string())
        }
        443 => Some("HTTPS".to_string()),
        53 => Some("DNS".to_string()),
        22 => Some("SSH".to_string()),
        21 => Some("FTP".to_string()),
        25 => Some("SMTP".to_string()),
        110 => Some("POP3".to_string()),
        143 => Some("IMAP".to_string()),
        993 => Some("IMAPS".to_string()),
        995 => Some("POP3S".to_string()),
        _ => None,
    }
}

fn detect_threat_level(packet_info: &PacketInfo) -> ThreatLevel {
    // Sophisticated threat detection based on multiple factors
    let mut risk_score = 0;
    
    // Check for suspicious ports
    if let Some(port) = packet_info.dst_port.or(packet_info.src_port) {
        match port {
            // High-risk ports
            1433 | 3389 | 5900 | 23 | 135 | 139 | 445 => risk_score += 3,
            // Medium-risk ports  
            21 | 25 | 110 | 143 | 993 | 995 => risk_score += 2,
            // Unusual high ports
            p if p > 49152 => risk_score += 1,
            _ => {}
        }
    }
    
    // Check for suspicious IP patterns
    if let Some(ref ip) = packet_info.dst_ip {
        // Private IP ranges are generally safer
        if !is_private_ip(ip) {
            risk_score += 1;
        }
        
        // Check for known malicious patterns (simplified)
        if ip.starts_with("10.0.0.") || ip.starts_with("169.254.") {
            risk_score += 2;
        }
    }
    
    // Check packet size anomalies
    if packet_info.packet_size > 1500 || packet_info.packet_size < 64 {
        risk_score += 1;
    }
    
    // Check for suspicious protocols
    match packet_info.protocol.as_str() {
        "ICMP" => risk_score += 1, // Could be scanning
        "UDP" if packet_info.dst_port == Some(53) => {}, // DNS is normal
        "UDP" => risk_score += 1, // Other UDP could be suspicious
        _ => {}
    }
    
    // Convert risk score to threat level
    match risk_score {
        0..=1 => ThreatLevel::Safe,
        2..=3 => ThreatLevel::Low,
        4..=5 => ThreatLevel::Medium,
        6..=7 => ThreatLevel::High,
        _ => ThreatLevel::Critical,
    }
}

fn is_private_ip(ip: &str) -> bool {
    ip.starts_with("10.") || 
    ip.starts_with("192.168.") || 
    ip.starts_with("172.16.") ||
    ip.starts_with("127.") ||
    ip.starts_with("::1") ||
    ip.starts_with("fe80::")
}

fn get_geo_info(ip: &str) -> Option<GeoInfo> {
    // Simplified geolocation (in production, use MaxMind GeoIP2 or similar)
    if is_private_ip(ip) {
        return Some(GeoInfo {
            country: Some("Local Network".to_string()),
            city: Some("Local".to_string()),
            latitude: None,
            longitude: None,
        });
    }
    
    // For demo purposes, return some sample data based on IP patterns
    match ip {
        ip if ip.starts_with("8.8.") => Some(GeoInfo {
            country: Some("United States".to_string()),
            city: Some("Mountain View".to_string()),
            latitude: Some(37.4056),
            longitude: Some(-122.0775),
        }),
        ip if ip.starts_with("1.1.") => Some(GeoInfo {
            country: Some("Australia".to_string()),
            city: Some("Sydney".to_string()),
            latitude: Some(-33.8688),
            longitude: Some(151.2093),
        }),
        _ => Some(GeoInfo {
            country: Some("Unknown".to_string()),
            city: Some("Unknown".to_string()),
            latitude: None,
            longitude: None,
        }),
    }
}

fn format_packet_description(packet_info: &PacketInfo) -> String {
    match packet_info.application_protocol.as_ref() {
        Some(app_proto) => {
            match app_proto.as_str() {
                "HTTP" => "Web browsing (HTTP request/response)".to_string(),
                "HTTPS" => "Secure web browsing (encrypted)".to_string(),
                "DNS" => "Domain name lookup".to_string(),
                "SSH" => "Secure shell connection".to_string(),
                "FTP" => "File transfer".to_string(),
                "SMTP" => "Email sending".to_string(),
                "Web Traffic" => "Web-related traffic".to_string(),
                _ => format!("{} communication", app_proto),
            }
        }
        None => {
            match packet_info.protocol.as_str() {
                "TCP" => {
                    if let (Some(src_port), Some(dst_port)) = (packet_info.src_port, packet_info.dst_port) {
                        format!("TCP connection from port {} to port {}", src_port, dst_port)
                    } else {
                        "TCP connection".to_string()
                    }
                }
                "UDP" => {
                    if let (Some(src_port), Some(dst_port)) = (packet_info.src_port, packet_info.dst_port) {
                        format!("UDP communication from port {} to port {}", src_port, dst_port)
                    } else {
                        "UDP communication".to_string()
                    }
                }
                "ICMP" => "Network diagnostic (ping/traceroute)".to_string(),
                _ => format!("{} network traffic", packet_info.protocol),
            }
        }
    }
}

fn start_sniffing(interface: NetworkInterface, args: Args, config: Config, mut logger: Logger) -> Result<()> {
    use pnet::datalink::Channel::Ethernet;
    
    let start_time = Instant::now();
    
    println!("{}", "🚀 Starting Advanced Packet Capture".green().bold());
    println!("{}", format!("📡 Interface: {}", interface.name).cyan());
    if let Some(ref protocol) = args.protocol {
        println!("{}", format!("🔍 Protocol Filter: {}", protocol).yellow());
    }
    if let Some(port) = args.port {
        println!("{}", format!("🚪 Port Filter: {}", port).yellow());
    }
    if args.count > 0 {
        println!("{}", format!("📊 Capture Limit: {} packets", args.count).blue());
    }
    
    let mut captured_packets = Vec::<PacketInfo>::new();
    let stats_start = Instant::now();
    
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            return Err(PacketSnifferError::NetworkError("Unhandled channel type".to_string()));
        }
        Err(e) => {
            return Err(PacketSnifferError::NetworkError(format!("Failed to create datalink channel: {}", e)));
        }
    };
    
    println!("{}", "🎯 Capturing packets... (Press Ctrl+C to stop)".green());
    println!();
    
    let mut packet_count = 0;
    let mut last_stats_time = Instant::now();
    
    loop {
        if args.count > 0 && packet_count >= args.count {
            break;
        }
        
        match rx.next() {
            Ok(packet) => {
                if should_capture_packet(packet, &args) {
                    let packet_info = analyze_packet_advanced(packet, packet_count + 1);
                    
                    if args.verbose {
                        display_packet_verbose(&packet_info);
                    } else {
                        display_packet_simple(&packet_info);
                    }
                    
                    captured_packets.push(packet_info);
                    packet_count += 1;
                    
                    // Show periodic stats
                    if last_stats_time.elapsed().as_secs() >= args.stats_interval {
                        display_interim_stats(&captured_packets, stats_start.elapsed());
                        last_stats_time = Instant::now();
                    }
                }
            }
            Err(e) => {
                println!("{}", format!("❌ Failed to read packet: {}", e).red());
                break;
            }
        }
    }
    
    // Final summary
    display_final_summary(&captured_packets, stats_start.elapsed());
    
    // Export if requested
    if let Some(ref json_file) = args.export_json {
        export_to_json(&captured_packets, json_file)?;
        logger.log_export("JSON", json_file, captured_packets.len());
    }
    
    if let Some(ref csv_file) = args.export_csv {
        export_to_csv(&captured_packets, csv_file)?;
        logger.log_export("CSV", csv_file, captured_packets.len());
    }
    
    logger.log_packet_capture_stop(captured_packets.len(), start_time.elapsed().as_secs());
    Ok(())
}

fn display_packet_simple(packet_info: &PacketInfo) {
    let timestamp = packet_info.timestamp.format("%H:%M:%S%.3f").to_string();
    let src = packet_info.src_ip.as_ref().map(|s| s.as_str()).unwrap_or("N/A");
    let dst = packet_info.dst_ip.as_ref().map(|s| s.as_str()).unwrap_or("N/A");
    
    println!("🕐 {} | {} {} | {} -> {} | {}", 
             timestamp.cyan(),
             packet_info.protocol.green().bold(),
             packet_info.application_protocol.as_ref().unwrap_or(&"".to_string()).yellow(),
             src.blue(),
             dst.blue(),
             packet_info.description.white());
}

fn display_packet_verbose(packet_info: &PacketInfo) {
    println!("{}", format!("[Packet #{}]", packet_info.packet_number).bold().green());
    println!("🕐 Timestamp: {}", packet_info.timestamp.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string().cyan());
    println!("📟 Ethernet: {} -> {}", packet_info.src_mac.blue(), packet_info.dst_mac.blue());
    
    if let (Some(src_ip), Some(dst_ip)) = (&packet_info.src_ip, &packet_info.dst_ip) {
        println!("🌐 IP: {} -> {} ({})", src_ip.green(), dst_ip.green(), packet_info.protocol.yellow());
    }
    
    if let (Some(src_port), Some(dst_port)) = (packet_info.src_port, packet_info.dst_port) {
        println!("🚪 Ports: {} -> {}", src_port.to_string().magenta(), dst_port.to_string().magenta());
    }
    
    if let Some(ref flags) = packet_info.flags {
        println!("🏁 Flags: {}", flags.red());
    }
    
    if let Some(ref app_proto) = packet_info.application_protocol {
        println!("📱 Application: {}", app_proto.bright_yellow().bold());
    }
    
    println!("📊 Size: {} bytes (payload: {} bytes)", packet_info.packet_size, packet_info.payload_size);
    println!("💬 Description: {}", packet_info.description.italic());
    println!("{}", "─".repeat(80).bright_black());
}

fn display_interim_stats(packets: &[PacketInfo], duration: Duration) {
    println!("\n{}", "📈 Interim Statistics".bright_green().bold());
    println!("{}", "═".repeat(50).blue());
    
    let duration_secs = duration.as_secs();
    let total_packets = packets.len();
    let total_bytes: usize = packets.iter().map(|p| p.packet_size).sum();
    
    println!("⏱️  Duration: {}s | 📦 Packets: {} ({:.1}/s)", 
             duration_secs, total_packets, 
             total_packets as f64 / duration_secs as f64);
    println!("📊 Total Data: {}", format_bytes(total_bytes));
    
    // Protocol breakdown
    let mut protocol_counts = HashMap::new();
    for packet in packets {
        *protocol_counts.entry(packet.protocol.clone()).or_insert(0) += 1;
    }
    
    println!("🔗 Protocols:");
    for (protocol, count) in protocol_counts {
        println!("   {} {}: {}", "▶".green(), protocol.yellow(), count);
    }
    
    println!("{}", "═".repeat(50).blue());
    println!();
}

fn display_final_summary(packets: &[PacketInfo], duration: Duration) {
    println!("\n{}", "🏁 Capture Complete - Final Summary".bright_green().bold());
    println!("{}", "═".repeat(80).blue());
    
    let duration_secs = duration.as_secs();
    let total_packets = packets.len();
    let total_bytes: usize = packets.iter().map(|p| p.packet_size).sum();
    
    println!("⏱️  Total Duration: {}s", duration_secs);
    println!("📦 Total Packets: {} ({:.2} packets/second)", 
             total_packets, total_packets as f64 / duration_secs as f64);
    println!("📊 Total Data: {} ({:.2} bytes/second)", 
             format_bytes(total_bytes), total_bytes as f64 / duration_secs as f64);
    
    // Detailed protocol statistics
    let mut protocol_counts = HashMap::new();
    let mut app_protocol_counts = HashMap::new();
    
    for packet in packets {
        *protocol_counts.entry(packet.protocol.clone()).or_insert(0) += 1;
        if let Some(ref app_proto) = packet.application_protocol {
            *app_protocol_counts.entry(app_proto.clone()).or_insert(0) += 1;
        }
    }
    
    println!("\n{}", "🔗 Protocol Distribution:".yellow().bold());
    let mut protocol_table = Table::new();
    protocol_table.add_row(Row::new(vec![
        Cell::new("Protocol").style_spec("Fb"),
        Cell::new("Packets").style_spec("Fb"),
        Cell::new("Percentage").style_spec("Fb"),
    ]));
    
    for (protocol, count) in protocol_counts {
        let percentage = (count as f64 / total_packets as f64) * 100.0;
        protocol_table.add_row(Row::new(vec![
            Cell::new(&protocol),
            Cell::new(&count.to_string()),
            Cell::new(&format!("{:.1}%", percentage)),
        ]));
    }
    protocol_table.printstd();
    
    if !app_protocol_counts.is_empty() {
        println!("\n{}", "📱 Application Protocols:".yellow().bold());
        let mut app_table = Table::new();
        app_table.add_row(Row::new(vec![
            Cell::new("Application").style_spec("Fb"),
            Cell::new("Packets").style_spec("Fb"),
            Cell::new("Percentage").style_spec("Fb"),
        ]));
        
        for (app_proto, count) in app_protocol_counts {
            let percentage = (count as f64 / total_packets as f64) * 100.0;
            app_table.add_row(Row::new(vec![
                Cell::new(&app_proto),
                Cell::new(&count.to_string()),
                Cell::new(&format!("{:.1}%", percentage)),
            ]));
        }
        app_table.printstd();
    }
    
    println!("{}", "═".repeat(80).blue());
}

fn export_to_json(packets: &[PacketInfo], filename: &str) -> Result<()> {
    let json_data = serde_json::to_string_pretty(packets)
        .map_err(|e| PacketSnifferError::ExportError(format!("Failed to serialize data: {}", e)))?;
    
    std::fs::write(filename, json_data)
        .map_err(|e| PacketSnifferError::ExportError(format!("Failed to write JSON file: {}", e)))?;
    
    println!("{}", format!("✅ Exported {} packets to {}", packets.len(), filename).green());
    Ok(())
}

fn export_to_csv(packets: &[PacketInfo], filename: &str) -> Result<()> {
    let mut wtr = csv::Writer::from_path(filename)
        .map_err(|e| PacketSnifferError::ExportError(format!("Failed to create CSV file: {}", e)))?;
    
    // Write header
    wtr.write_record(&["timestamp", "packet_number", "src_ip", "dst_ip", "protocol", 
                       "src_port", "dst_port", "packet_size", "flags", "application_protocol", "description"])
        .map_err(|e| PacketSnifferError::ExportError(format!("Failed to write CSV header: {}", e)))?;
    
    // Write data
    for packet in packets {
        let record = vec![
            packet.timestamp.to_rfc3339(),
            packet.packet_number.to_string(),
            packet.src_ip.as_ref().unwrap_or(&"".to_string()).clone(),
            packet.dst_ip.as_ref().unwrap_or(&"".to_string()).clone(),
            packet.protocol.clone(),
            packet.src_port.map_or("".to_string(), |p| p.to_string()),
            packet.dst_port.map_or("".to_string(), |p| p.to_string()),
            packet.packet_size.to_string(),
            packet.flags.as_ref().unwrap_or(&"".to_string()).clone(),
            packet.application_protocol.as_ref().unwrap_or(&"".to_string()).clone(),
            packet.description.clone(),
        ];
        
        wtr.write_record(&record)
            .map_err(|e| PacketSnifferError::ExportError(format!("Failed to write CSV record: {}", e)))?;
    }
    
    wtr.flush()
        .map_err(|e| PacketSnifferError::ExportError(format!("Failed to flush CSV file: {}", e)))?;
    
    println!("{}", format!("✅ Exported {} packets to {}", packets.len(), filename).green());
    Ok(())
}

fn should_capture_packet(packet: &[u8], args: &Args) -> bool {
    if let Some(ethernet_packet) = EthernetPacket::new(packet) {
        match ethernet_packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    // Check protocol filter
                    if let Some(ref protocol_filter) = args.protocol {
                        let protocol_match = match protocol_filter.to_lowercase().as_str() {
                            "tcp" => ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Tcp,
                            "udp" => ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Udp,
                            "icmp" => ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Icmp,
                            "http" => {
                                // Check if it's TCP on port 80 or 8080
                                if ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Tcp {
                                    if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                        return tcp_packet.get_destination() == 80 || tcp_packet.get_source() == 80 ||
                                               tcp_packet.get_destination() == 8080 || tcp_packet.get_source() == 8080;
                                    }
                                }
                                return false;
                            }
                            "dns" => {
                                // Check if it's UDP on port 53
                                if ipv4_packet.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Udp {
                                    if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                        return udp_packet.get_destination() == 53 || udp_packet.get_source() == 53;
                                    }
                                }
                                return false;
                            }
                            _ => true,
                        };
                        
                        if !protocol_match {
                            return false;
                        }
                    }
                    
                    // Check port filter
                    if let Some(port_filter) = args.port {
                        match ipv4_packet.get_next_level_protocol() {
                            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                    return tcp_packet.get_source() == port_filter || tcp_packet.get_destination() == port_filter;
                                }
                            }
                            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                                if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                    return udp_packet.get_source() == port_filter || udp_packet.get_destination() == port_filter;
                                }
                            }
                            _ => return false,
                        }
                    }
                }
            }
            _ => return false,
        }
    }
    
    true
}


