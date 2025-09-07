# Packet Sniffer

A network packet sniffer written in Rust that captures and analyzes network packets in real-time.

## Features

- **Network Interface Discovery**: List all available network interfaces
- **Protocol Filtering**: Filter packets by protocol (TCP, UDP, ICMP)
- **Port Filtering**: Filter packets by specific port numbers
- **Packet Counting**: Capture a specific number of packets or run continuously
- **Detailed Packet Analysis**: Parse and display Ethernet, IPv4, TCP, and UDP packet information
- **Real-time Capture**: Live packet monitoring with formatted output

## Usage

### List Available Network Interfaces

```bash
cargo run -- --list-interfaces
```

### Basic Packet Capture

Capture packets on a specific interface:

```bash
sudo cargo run -- --interface eth0
```

### Filter by Protocol

Capture only TCP packets:

```bash
sudo cargo run -- --interface eth0 --protocol tcp
```

Capture only UDP packets:

```bash
sudo cargo run -- --interface eth0 --protocol udp
```

### Filter by Port

Capture packets on port 80 (HTTP):

```bash
sudo cargo run -- --interface eth0 --port 80
```

### Capture Limited Number of Packets

Capture only 10 packets:

```bash
sudo cargo run -- --interface eth0 --count 10
```

### Combined Filters

Capture 5 TCP packets on port 443 (HTTPS):

```bash
sudo cargo run -- --interface eth0 --protocol tcp --port 443 --count 5
```

## Command Line Options

- `-i, --interface <INTERFACE>`: Network interface to sniff on
- `-p, --protocol <PROTOCOL>`: Filter by protocol (tcp, udp, icmp)
- `-P, --port <PORT>`: Filter by port number
- `-c, --count <COUNT>`: Number of packets to capture (0 = unlimited)
- `-l, --list-interfaces`: Show available network interfaces
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Requirements

- Rust 1.70+ 
- Root privileges (sudo) for packet capture
- Network interface access

## Dependencies

- `pnet`: For low-level packet capture and parsing
- `clap`: For command-line argument parsing
- `tokio`: For async runtime support

## Building

```bash
cargo build --release
```

## Installation

```bash
cargo install --path .
```

## Security Note

This tool requires root privileges to capture network packets. Always use with caution and only on networks you own or have permission to monitor.

## Sample Output

```
$ sudo cargo run -- --interface eth0 --count 3

Starting packet capture on interface: eth0

[Packet #1]
Ethernet: 52:54:00:12:34:56 -> 08:00:27:ab:cd:ef
IPv4: 192.168.1.100 -> 93.184.216.34 (Protocol: Tcp)
TCP: Port 54321 -> 80 [Flags: 0x18]
  Flags: PSH ACK 
  Data: 76 bytes

[Packet #2]
Ethernet: 08:00:27:ab:cd:ef -> 52:54:00:12:34:56
IPv4: 93.184.216.34 -> 192.168.1.100 (Protocol: Tcp)
TCP: Port 80 -> 54321 [Flags: 0x18]
  Flags: PSH ACK 
  Data: 1448 bytes

[Packet #3]
Ethernet: 52:54:00:12:34:56 -> 08:00:27:ab:cd:ef
IPv4: 192.168.1.100 -> 8.8.8.8 (Protocol: Udp)
UDP: Port 54322 -> 53 Length: 40
  Data: 32 bytes

Captured 3 packets
```