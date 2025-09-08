# Installation Guide for Advanced Network Packet Sniffer

## System Requirements

- **Operating System**: Windows 10/11 (64-bit)
- **Administrator Privileges**: Required for packet capture
- **Network Interface**: Active network connection
- **RAM**: Minimum 4GB recommended
- **Disk Space**: 50MB for installation

## Installation Steps

1. **Download**: Download the installer `PacketSnifferSetup-1.0.0.exe`
2. **Run as Administrator**: Right-click the installer and select "Run as administrator"
3. **Follow Wizard**: Complete the installation wizard
4. **Configuration**: The installer will create a default configuration file

## Post-Installation

### First Run
1. Open Command Prompt as Administrator
2. Navigate to installation directory (default: `C:\Program Files\Advanced Network Packet Sniffer\`)
3. Run: `packet_sniffer.exe --list-interfaces` to see available network interfaces
4. Run: `packet_sniffer.exe --interface "Your Network Interface" --dashboard` to start monitoring

### Configuration
- Configuration file location: `%APPDATA%\packet_sniffer\config.json`
- Log files location: `%APPDATA%\packet_sniffer\logs\`
- Export files location: `%APPDATA%\packet_sniffer\exports\`

### Common Commands
```cmd
REM List available network interfaces
packet_sniffer.exe --list-interfaces

REM Start interactive dashboard
packet_sniffer.exe --interface "Ethernet" --dashboard

REM Monitor web traffic
packet_sniffer.exe --interface "Ethernet" --protocol http --verbose

REM Export traffic data
packet_sniffer.exe --interface "Ethernet" --count 100 --export-json traffic.json
```

## Troubleshooting

### "Access Denied" Errors
- Ensure you're running as Administrator
- Check Windows Defender/Antivirus settings
- Verify network interface is active

### Interface Not Found
- Use `--list-interfaces` to see exact interface names
- Interface names are case-sensitive
- Ensure network adapter is enabled

### Performance Issues
- Reduce capture count with `--count` parameter
- Use protocol filters to limit captured data
- Check available disk space for export files

## Security Notice

This application captures network packets and requires administrator privileges. Only use on networks you own or have explicit permission to monitor. The application does not send any data externally - all analysis is performed locally.

## Support

For issues and questions, please refer to:
- README.md for detailed usage instructions
- examples.md for practical examples
- GitHub repository for bug reports and feature requests

## Uninstallation

To remove the application:
1. Go to Windows Settings > Apps
2. Find "Advanced Network Packet Sniffer" 
3. Click Uninstall
4. Manually delete configuration files from `%APPDATA%\packet_sniffer\` if desired