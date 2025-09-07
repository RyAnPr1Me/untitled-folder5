use clap::Parser;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Network interface to sniff on
    #[arg(short, long)]
    interface: Option<String>,
    
    /// Filter by protocol (tcp, udp, icmp)
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
            eprintln!("No interface specified. Use --list-interfaces to see available interfaces.");
            return;
        }
    };
    
    let interface = match find_interface(&interface_name) {
        Some(iface) => iface,
        None => {
            eprintln!("Interface '{}' not found. Use --list-interfaces to see available interfaces.", interface_name);
            return;
        }
    };
    
    start_sniffing(interface, args);
}

fn list_interfaces() {
    println!("Available network interfaces:");
    for interface in datalink::interfaces() {
        println!("  {}: {}", interface.name, interface.description);
        if !interface.ips.is_empty() {
            for ip in &interface.ips {
                println!("    IP: {}", ip);
            }
        }
    }
}

fn find_interface(name: &str) -> Option<NetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == name)
}

fn start_sniffing(interface: NetworkInterface, args: Args) {
    use pnet::datalink::Channel::Ethernet;
    
    println!("Starting packet capture on interface: {}", interface.name);
    if let Some(ref protocol) = args.protocol {
        println!("Filtering by protocol: {}", protocol);
    }
    if let Some(port) = args.port {
        println!("Filtering by port: {}", port);
    }
    
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
                    handle_packet(packet, packet_count + 1);
                    packet_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Failed to read packet: {}", e);
                break;
            }
        }
    }
    
    println!("\nCaptured {} packets", packet_count);
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

fn handle_packet(packet: &[u8], packet_num: usize) {
    if let Some(ethernet_packet) = EthernetPacket::new(packet) {
        println!("\n[Packet #{}]", packet_num);
        println!("Ethernet: {} -> {}", 
                 ethernet_packet.get_source(), 
                 ethernet_packet.get_destination());
        
        match ethernet_packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                handle_ipv4_packet(ethernet_packet.payload());
            }
            EtherTypes::Ipv6 => {
                println!("IPv6 packet (parsing not implemented)");
            }
            _ => {
                println!("Unknown ethernet type: {:?}", ethernet_packet.get_ethertype());
            }
        }
    }
}

fn handle_ipv4_packet(packet: &[u8]) {
    if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
        println!("IPv4: {} -> {} (Protocol: {:?})", 
                 ipv4_packet.get_source(), 
                 ipv4_packet.get_destination(),
                 ipv4_packet.get_next_level_protocol());
        
        match ipv4_packet.get_next_level_protocol() {
            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                handle_tcp_packet(ipv4_packet.payload());
            }
            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                handle_udp_packet(ipv4_packet.payload());
            }
            pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                println!("ICMP packet");
            }
            _ => {
                println!("Unknown IP protocol");
            }
        }
    }
}

fn handle_tcp_packet(packet: &[u8]) {
    if let Some(tcp_packet) = TcpPacket::new(packet) {
        let flags = tcp_packet.get_flags();
        println!("TCP: Port {} -> {} [Flags: 0x{:02x}] Seq: {} Ack: {} Window: {}",
                 tcp_packet.get_source(),
                 tcp_packet.get_destination(),
                 flags,
                 tcp_packet.get_sequence(),
                 tcp_packet.get_acknowledgement(),
                 tcp_packet.get_window());
        
        // Decode TCP flags
        let mut flag_str = String::new();
        if flags & 0x01 != 0 { flag_str.push_str("FIN "); }
        if flags & 0x02 != 0 { flag_str.push_str("SYN "); }
        if flags & 0x04 != 0 { flag_str.push_str("RST "); }
        if flags & 0x08 != 0 { flag_str.push_str("PSH "); }
        if flags & 0x10 != 0 { flag_str.push_str("ACK "); }
        if flags & 0x20 != 0 { flag_str.push_str("URG "); }
        if flags & 0x40 != 0 { flag_str.push_str("ECE "); }
        if flags & 0x80 != 0 { flag_str.push_str("CWR "); }
        
        if !flag_str.is_empty() {
            println!("  Flags: {}", flag_str.trim());
        }
        
        if !tcp_packet.payload().is_empty() {
            println!("  Data: {} bytes", tcp_packet.payload().len());
        }
    }
}

fn handle_udp_packet(packet: &[u8]) {
    if let Some(udp_packet) = UdpPacket::new(packet) {
        println!("UDP: Port {} -> {} Length: {}",
                 udp_packet.get_source(),
                 udp_packet.get_destination(),
                 udp_packet.get_length());
        
        if !udp_packet.payload().is_empty() {
            println!("  Data: {} bytes", udp_packet.payload().len());
        }
    }
}
