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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
}

#[derive(Debug, Clone)]
struct NetworkStats {
    total_packets: usize,
    total_bytes: usize,
    protocol_counts: HashMap<String, usize>,
    top_talkers: HashMap<String, usize>,
    start_time: Instant,
}

fn main() {
    let args = Args::parse();
    
    if args.list_interfaces {
        list_interfaces();
        return;
    }
    
    let interface_name = match &args.interface {
        Some(name) => name.clone(),
        None => {
            println!("{}", "❌ No interface specified.".red().bold());
            println!("{}", "💡 Use --list-interfaces to see available interfaces.".yellow());
            println!("{}", "📖 Example: sudo cargo run -- --interface eth0".cyan());
            return;
        }
    };
    
    let interface = match find_interface(&interface_name) {
        Some(iface) => iface,
        None => {
            println!("{}", format!("❌ Interface '{}' not found.", interface_name).red().bold());
            println!("{}", "💡 Use --list-interfaces to see available interfaces.".yellow());
            return;
        }
    };
    
    if args.dashboard {
        start_dashboard_mode(interface, args);
    } else {
        start_sniffing(interface, args);
    }
}

fn list_interfaces() {
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

fn start_dashboard_mode(interface: NetworkInterface, args: Args) {
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
    }
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
                        
                        if let Some(src_ip) = &packet_info.src_ip {
                            *stats.top_talkers.entry(src_ip.clone()).or_insert(0) += 1;
                        }
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
    
    println!("{}", "📊 Network Traffic Dashboard".green().bold());
    println!("{}", "═".repeat(80).blue());
    
    let duration = stats.start_time.elapsed().as_secs();
    let packets_per_sec = if duration > 0 { stats.total_packets as f64 / duration as f64 } else { 0.0 };
    let bytes_per_sec = if duration > 0 { stats.total_bytes as f64 / duration as f64 } else { 0.0 };
    
    // Summary stats
    println!("⏱️  Duration: {}s | 📦 Packets: {} ({:.1}/s) | 📊 Bytes: {} ({:.1}/s)", 
             duration, stats.total_packets, packets_per_sec, 
             format_bytes(stats.total_bytes), bytes_per_sec);
    println!();
    
    // Protocol distribution
    println!("{}", "🔗 Protocol Distribution:".yellow().bold());
    let mut protocol_table = Table::new();
    protocol_table.add_row(Row::new(vec![
        Cell::new("Protocol").style_spec("Fb"),
        Cell::new("Packets").style_spec("Fb"),
        Cell::new("Percentage").style_spec("Fb"),
    ]));
    
    for (protocol, count) in &stats.protocol_counts {
        let percentage = (*count as f64 / stats.total_packets as f64) * 100.0;
        protocol_table.add_row(Row::new(vec![
            Cell::new(protocol),
            Cell::new(&count.to_string()),
            Cell::new(&format!("{:.1}%", percentage)),
        ]));
    }
    protocol_table.printstd();
    
    // Recent packets
    println!("\n{}", "📋 Recent Packets:".yellow().bold());
    if packets.len() > 0 {
        let recent_packets: Vec<_> = packets.iter().rev().take(5).collect();
        for packet in recent_packets {
            let timestamp = packet.timestamp.format("%H:%M:%S").to_string();
            println!("🕐 {} | {} | {} | {}", 
                     timestamp.cyan(),
                     packet.protocol.green(),
                     format!("{} -> {}", 
                            packet.src_ip.as_ref().unwrap_or(&"N/A".to_string()),
                            packet.dst_ip.as_ref().unwrap_or(&"N/A".to_string())).blue(),
                     packet.description.white());
        }
    } else {
        println!("No packets captured yet...");
    }
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

fn start_sniffing(interface: NetworkInterface, args: Args) {
    use pnet::datalink::Channel::Ethernet;
    
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
            println!("{}", "❌ Unhandled channel type".red());
            return;
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to create datalink channel: {}", e).red());
            return;
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
        export_to_json(&captured_packets, json_file);
    }
    
    if let Some(ref csv_file) = args.export_csv {
        export_to_csv(&captured_packets, csv_file);
    }
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

fn export_to_json(packets: &[PacketInfo], filename: &str) {
    match serde_json::to_string_pretty(packets) {
        Ok(json_data) => {
            if std::fs::write(filename, json_data).is_ok() {
                println!("{}", format!("✅ Exported {} packets to {}", packets.len(), filename).green());
            } else {
                println!("{}", format!("❌ Failed to write JSON file: {}", filename).red());
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to serialize data: {}", e).red());
        }
    }
}

fn export_to_csv(packets: &[PacketInfo], filename: &str) {
    let mut wtr = match csv::Writer::from_path(filename) {
        Ok(writer) => writer,
        Err(e) => {
            println!("{}", format!("❌ Failed to create CSV file: {}", e).red());
            return;
        }
    };
    
    // Write header
    if wtr.write_record(&["timestamp", "packet_number", "src_ip", "dst_ip", "protocol", 
                         "src_port", "dst_port", "packet_size", "flags", "application_protocol", "description"]).is_err() {
        println!("{}", "❌ Failed to write CSV header".red());
        return;
    }
    
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
        
        if wtr.write_record(&record).is_err() {
            println!("{}", "❌ Failed to write CSV record".red());
            return;
        }
    }
    
    if wtr.flush().is_ok() {
        println!("{}", format!("✅ Exported {} packets to {}", packets.len(), filename).green());
    } else {
        println!("{}", format!("❌ Failed to finalize CSV file: {}", filename).red());
    }
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


