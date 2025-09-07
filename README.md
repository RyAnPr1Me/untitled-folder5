# 🌐 Advanced Network Packet Sniffer

A powerful, user-friendly network packet analyzer written in Rust that captures and analyzes network traffic in real-time. Designed to make network analysis accessible to both technical experts and everyday users.

## ✨ What Makes This Special

This isn't just another packet sniffer - it's designed to **explain what's happening on your network in plain English**:

- 🔍 **Smart Protocol Detection**: Automatically identifies HTTP, HTTPS, DNS, SSH, and more
- 💬 **Human-Readable Explanations**: "Web browsing" instead of "TCP port 80"
- 🎨 **Beautiful Output**: Color-coded, organized display with emojis for quick recognition
- 📊 **Real-Time Dashboard**: Live statistics and network activity monitoring
- 📁 **Easy Export**: Save data in JSON or CSV for further analysis
- 🎛️ **Flexible Filtering**: Focus on specific protocols, ports, or applications

## 🚀 Quick Start

### Installation & Basic Usage
```bash
# Clone and build
git clone <repository-url>
cd packet_sniffer
cargo build --release

# See available network interfaces
sudo cargo run -- --list-interfaces

# Start capturing with dashboard
sudo cargo run -- --interface eth0 --dashboard

# Capture specific traffic
sudo cargo run -- --interface eth0 --protocol http --verbose
```

## 🎯 Perfect For

- **Network Troubleshooting**: "Why is my internet slow?"
- **Security Monitoring**: "What's my computer talking to?"
- **Learning**: "How does the internet actually work?"
- **Development**: "Is my application sending the right requests?"

## 📖 Detailed Examples

See [examples.md](./examples.md) for comprehensive usage examples and explanations.

## 🛠️ Features

### Core Capabilities
- **Multi-Protocol Support**: Ethernet, IPv4, TCP, UDP, ICMP
- **Application Layer Detection**: HTTP, HTTPS, DNS, SSH, FTP, SMTP, and more
- **Real-Time Analysis**: Process packets as they're captured
- **Interactive Dashboard**: Live statistics with automatic refresh

### User-Friendly Features
- **Plain English Descriptions**: Each packet explained in human terms
- **Color-Coded Output**: Protocols, IPs, and data highlighted for easy reading
- **Smart Filtering**: Filter by protocol name (not just numbers)
- **Progress Indicators**: Know exactly what's happening

### Advanced Analysis
- **Statistics Tracking**: Bandwidth usage, protocol distribution, top talkers
- **Export Options**: JSON for programming, CSV for spreadsheets
- **Connection Tracking**: See conversation flows between hosts
- **Performance Metrics**: Packets per second, data rates

## 🎨 Example Output

### Simple Mode
```
🕐 14:30:15.123 | TCP HTTP | 192.168.1.100 -> 93.184.216.34 | Web browsing (HTTP request/response)
🕐 14:30:15.456 | UDP DNS | 192.168.1.100 -> 8.8.8.8 | Domain name lookup
🕐 14:30:15.789 | ICMP | 192.168.1.100 -> 8.8.8.8 | Network diagnostic (ping/traceroute)
```

### Dashboard Mode
```
📊 Network Traffic Dashboard
═══════════════════════════════════════════════════════════════════════════════
⏱️  Duration: 45s | 📦 Packets: 1,247 (27.7/s) | 📊 Bytes: 1.2 MB (27.3 KB/s)

🔗 Protocol Distribution:
┌──────────┬─────────┬────────────┐
│ Protocol │ Packets │ Percentage │
├──────────┼─────────┼────────────┤
│ TCP      │ 856     │ 68.6%      │
│ UDP      │ 312     │ 25.0%      │
│ ICMP     │ 79      │ 6.3%       │
└──────────┴─────────┴────────────┘

📋 Recent Packets:
🕐 14:30:22.891 | TCP | 192.168.1.100 -> 151.101.1.140 | Secure web browsing (encrypted)
🕐 14:30:22.654 | UDP | 192.168.1.100 -> 8.8.8.8 | Domain name lookup
```

## 🔧 Command Line Options

```bash
Advanced Network Packet Sniffer

Usage: packet_sniffer [OPTIONS]

Options:
  -i, --interface <INTERFACE>      Network interface to sniff on
  -p, --protocol <PROTOCOL>        Filter by protocol (tcp, udp, icmp, http, dns)
  -P, --port <PORT>               Filter by port number
  -c, --count <COUNT>             Number of packets to capture (0 = unlimited) [default: 0]
  -l, --list-interfaces           Show available network interfaces
  -d, --dashboard                 Enable interactive dashboard mode
      --export-json <EXPORT_JSON> Export captured data to JSON file
      --export-csv <EXPORT_CSV>   Export captured data to CSV file
  -v, --verbose                   Show detailed packet analysis
      --stats-interval <STATS_INTERVAL> Show statistics summary every N seconds [default: 10]
  -h, --help                      Print help
  -V, --version                   Print version
```

## 💡 Use Cases & Examples

### For Network Troubleshooting
```bash
# See all network activity at a glance
sudo cargo run -- --interface eth0 --dashboard

# Focus on web traffic issues
sudo cargo run -- --interface eth0 --protocol http --verbose
```

### For Security Monitoring
```bash
# Monitor DNS queries (see what domains are being accessed)
sudo cargo run -- --interface eth0 --protocol dns --export-json dns_log.json

# Comprehensive traffic logging
sudo cargo run -- --interface eth0 --count 1000 --export-csv security_audit.csv
```

### For Learning & Education
```bash
# Understand how web browsing works
sudo cargo run -- --interface eth0 --protocol http --verbose

# See the DNS resolution process
sudo cargo run -- --interface eth0 --protocol dns --verbose
```

## 🔒 Security & Privacy

- **Requires administrator privileges** for raw packet access
- **Only analyze networks you own** or have explicit permission to monitor
- **No data is sent externally** - all analysis happens locally
- **Export files contain sensitive data** - handle appropriately

## 🏗️ Technical Details

- **Language**: Rust (for performance and safety)
- **Dependencies**: Uses `pnet` for packet capture, `clap` for CLI, `colored` for output
- **Platform**: Linux, macOS, Windows (with appropriate permissions)
- **Performance**: Optimized for real-time packet processing

## 🤝 Contributing

We welcome contributions! Whether it's:
- Bug reports and feature requests
- Code improvements and new features
- Documentation and examples
- Testing on different platforms

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**🎓 Educational Note**: This tool is designed to help users understand network traffic and troubleshoot connectivity issues. Use responsibly and only on networks you own or have permission to monitor.