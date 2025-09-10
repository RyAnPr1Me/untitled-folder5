# Advanced Network Packet Sniffer - Usage Examples

## 🌟 Key Features

This advanced packet sniffer provides:

- **Real-time packet analysis** with human-readable explanations
- **Interactive dashboard mode** with live statistics
- **Application protocol detection** (HTTP, HTTPS, DNS, SSH, etc.)
- **Colored output** for better readability
- **Export capabilities** (JSON, CSV formats)
- **Advanced filtering** (protocol, port, application type)

## 🚀 Quick Start Examples

### 1. List Available Network Interfaces
```bash
sudo cargo run -- --list-interfaces
```

### 2. Basic Packet Capture
```bash
# Capture packets on ethernet interface
sudo cargo run -- --interface eth0

# Capture with dashboard mode (real-time statistics)
sudo cargo run -- --interface eth0 --dashboard
```

### 3. Protocol Filtering

#### Capture only HTTP traffic
```bash
sudo cargo run -- --interface eth0 --protocol http --verbose
```

#### Capture only DNS queries
```bash
sudo cargo run -- --interface eth0 --protocol dns
```

#### Capture TCP traffic
```bash
sudo cargo run -- --interface eth0 --protocol tcp --count 50
```

### 4. Port-based Filtering
```bash
# Capture web traffic (port 80)
sudo cargo run -- --interface eth0 --port 80

# Capture HTTPS traffic (port 443)
sudo cargo run -- --interface eth0 --port 443

# Capture SSH traffic (port 22)
sudo cargo run -- --interface eth0 --port 22
```

### 5. Advanced Analysis with Export
```bash
# Verbose mode with JSON export
sudo cargo run -- --interface eth0 --verbose --count 100 --export-json network_traffic.json

# Dashboard mode with CSV export
sudo cargo run -- --interface eth0 --dashboard --export-csv network_log.csv
```

### 6. Monitoring Specific Applications
```bash
# Monitor web browsing activity
sudo cargo run -- --interface eth0 --protocol http --verbose

# Monitor DNS lookups (see what websites are being accessed)
sudo cargo run -- --interface eth0 --protocol dns --stats-interval 5

# Monitor all traffic with periodic summaries
sudo cargo run -- --interface eth0 --stats-interval 10
```

## 📊 Understanding the Output

### Simple Mode Output
```
🕐 14:30:15.123 | TCP HTTP | 192.168.1.100 -> 93.184.216.34 | Web browsing (HTTP request/response)
🕐 14:30:15.456 | UDP DNS | 192.168.1.100 -> 8.8.8.8 | Domain name lookup
🕐 14:30:15.789 | TCP HTTPS | 192.168.1.100 -> 151.101.1.140 | Secure web browsing (encrypted)
```

### Verbose Mode Output
```
[Packet #1]
🕐 Timestamp: 2024-01-15 14:30:15.123 UTC
📟 Ethernet: aa:bb:cc:dd:ee:ff -> 11:22:33:44:55:66
🌐 IP: 192.168.1.100 -> 93.184.216.34 (TCP)
🚪 Ports: 54321 -> 80
🏁 Flags: PSH ACK
📱 Application: HTTP
📊 Size: 1514 bytes (payload: 1460 bytes)
💬 Description: Web browsing (HTTP request/response)
```

### Dashboard Mode Features
- **Real-time statistics** (packets per second, data rate)
- **Protocol distribution** (percentage breakdown)
- **Recent packet stream** (last 5 packets)
- **Top talkers** (most active IP addresses)

## 🎯 Common Use Cases

### Network Troubleshooting
```bash
# See all network activity
sudo cargo run -- --interface eth0 --dashboard

# Focus on connection issues
sudo cargo run -- --interface eth0 --protocol tcp --verbose
```

### Security Monitoring
```bash
# Monitor DNS queries (detect suspicious domains)
sudo cargo run -- --interface eth0 --protocol dns --export-json dns_log.json

# Monitor all traffic for analysis
sudo cargo run -- --interface eth0 --count 1000 --export-csv security_audit.csv
```

### Performance Analysis
```bash
# Real-time bandwidth monitoring
sudo cargo run -- --interface eth0 --dashboard --stats-interval 5

# Application-specific monitoring
sudo cargo run -- --interface eth0 --protocol http --stats-interval 3
```

### Educational/Learning
```bash
# See what happens when browsing websites
sudo cargo run -- --interface eth0 --protocol http --verbose

# Understand DNS resolution
sudo cargo run -- --interface eth0 --protocol dns --verbose
```

## 💡 Tips for Best Results

1. **Run with sudo**: Raw packet capture requires administrative privileges
2. **Use the right interface**: Check `--list-interfaces` to find your active network interface
3. **Start with dashboard mode**: Great for getting an overview of network activity
4. **Use filters wisely**: Focus on specific protocols or ports for targeted analysis
5. **Export for analysis**: Use JSON/CSV export for further processing with other tools
6. **Monitor specific timeframes**: Use `--count` to capture specific amounts of traffic

## 🔍 What Each Protocol Tells You

- **HTTP**: Web browsing, API calls, unencrypted web traffic
- **HTTPS**: Secure web browsing, encrypted communication
- **DNS**: Domain name lookups (shows what websites are being accessed)
- **TCP**: General connection-oriented communication
- **UDP**: Fast, connectionless communication (gaming, streaming, DNS)
- **ICMP**: Network diagnostics (ping, traceroute)

## 📈 Export Data Analysis

### JSON Export
Perfect for programmatic analysis, contains complete packet metadata including timestamps, protocols, and application details.

### CSV Export
Ideal for spreadsheet analysis, easy to import into Excel or Google Sheets for trend analysis and reporting.

## ⚠️ Important Notes

- Requires **root/administrator privileges** for raw socket access
- Monitor **only networks you own or have permission** to analyze
- Large captures can consume significant **disk space** and **memory**
- Use **filtering options** to focus on relevant traffic