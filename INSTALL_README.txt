# Installation Guide for Advanced Network Packet Sniffer

## 🚀 Quick Start

### System Requirements
- **Operating System**: Windows 10/11 (64-bit)
- **Administrator Privileges**: Required for packet capture
- **Dependencies**: Npcap driver (will be installed automatically)
- **Network Interface**: Active network connection
- **RAM**: Minimum 4GB recommended
- **Disk Space**: 100MB for installation

### Installation Steps

1. **Download**: Download the installer `PacketSnifferSetup-1.0.0.exe`
2. **Run as Administrator**: Right-click the installer and select "Run as administrator"
3. **Follow Installation Wizard**: 
   - Accept license agreement
   - Choose installation directory
   - Select optional features (desktop shortcuts, PATH integration)
   - Configure firewall exceptions if needed
4. **Dependency Setup**: The installer will check for and help install Npcap driver
5. **Configuration**: Default configuration files are created automatically

## 🎯 First Launch

### Method 1: Start Menu (Recommended)
1. Open Start Menu
2. Click "Advanced Network Packet Sniffer Dashboard"
3. Grant administrator permissions when prompted
4. The interactive dashboard will launch automatically

### Method 2: Desktop Shortcut
1. Double-click the desktop shortcut (if created during installation)
2. Choose "Dashboard Mode" for the best user experience

### Method 3: Command Line
1. Open Command Prompt as Administrator
2. Navigate to: `C:\Program Files\Advanced Network Packet Sniffer\`
3. Run: `packet_sniffer.exe --dashboard`

## 📁 File Locations

### Program Files
- **Installation**: `C:\Program Files\Advanced Network Packet Sniffer\`
- **Executable**: `packet_sniffer.exe`
- **Configuration**: `config\default_config.json`
- **Documentation**: `README.md`, `examples.md`, etc.

### User Data
- **Configuration**: `%APPDATA%\packet_sniffer\`
- **Log Files**: `%APPDATA%\packet_sniffer\logs\`
- **Export Files**: `%APPDATA%\packet_sniffer\exports\`

## 🖥️ Usage Examples

### Interactive Dashboard (Recommended for Beginners)
```cmd
packet_sniffer.exe --dashboard
```
Features:
- Real-time bandwidth graphs
- Security threat detection
- Plain English packet descriptions
- Connection flow visualization
- Live activity monitoring

### Command Line Operations
```cmd
REM List available network interfaces
packet_sniffer.exe --list-interfaces

REM Start monitoring specific interface
packet_sniffer.exe --interface "Wi-Fi" --dashboard

REM Monitor web traffic only
packet_sniffer.exe --interface "Ethernet" --protocol http --verbose

REM Export 100 packets to JSON
packet_sniffer.exe --interface "Ethernet" --count 100 --export-json traffic.json

REM Generate custom configuration
packet_sniffer.exe --generate-config --config my_config.json
```

## 🛡️ Security & Dependencies

### Npcap Driver Installation
The application requires Npcap for packet capture:
- **Automatic**: Installer will prompt you to install if not found
- **Manual**: Download from https://npcap.com/
- **Configuration**: Choose "WinPcap API-compatible Mode" if prompted

### Administrator Privileges Required
The application needs elevated privileges for:
- Raw network packet access
- Network interface interaction
- Security monitoring features

### Ethical Usage
⚠️ **Important**: Only use on networks you own or have permission to monitor:
- ✅ Your home/corporate networks (with approval)
- ✅ Educational environments (with authorization)
- ❌ Public networks without permission
- ❌ Networks owned by others

## 🔧 Troubleshooting

### Common Issues

#### "No network interfaces found"
**Solutions**:
1. Install/reinstall Npcap driver
2. Run as Administrator
3. Enable network interfaces in Device Manager
4. Check firewall settings

#### "Permission denied" errors
**Solutions**:
1. Right-click Command Prompt → "Run as administrator"
2. Check User Account Control (UAC) settings
3. Verify Npcap installation

#### Application won't start
**Solutions**:
1. Check Windows Event Viewer for errors
2. Verify all dependencies installed
3. Try: `packet_sniffer.exe --verbose --help`
4. Temporarily disable antivirus software

#### Interface names with spaces
**Solution**: Use quotes around interface names:
```cmd
packet_sniffer.exe --interface "Wi-Fi" --dashboard
```

### Performance Optimization
- Use `--count` to limit packet capture
- Apply protocol filters (`--protocol http`)
- Monitor disk space for export files
- Use dashboard mode for best user experience

## 🎨 Features Overview

### Advanced Dashboard GUI
- Real-time network visualization
- ASCII bandwidth graphs
- Threat detection with 5-level security indicators
- Connection flow tracking with geographical data
- Interactive statistics and performance metrics

### User-Friendly Output
Instead of technical data, see:
```
🕐 14:30:15.123 | TCP HTTP | 192.168.1.100 -> 93.184.216.34 | Web browsing (HTTP request/response)
🕐 14:30:15.456 | UDP DNS | 192.168.1.100 -> 8.8.8.8 | Domain name lookup
```

### Export Capabilities
- JSON format for programmatic analysis
- CSV format for spreadsheet use
- Real-time statistics and reporting

## 🔄 Updates & Maintenance

### Updating
1. Download new installer
2. Run installer (will update existing installation)
3. Configuration files are preserved

### Uninstalling
**Option 1 - Windows Settings**:
1. Settings → Apps → Advanced Network Packet Sniffer → Uninstall

**Option 2 - Control Panel**:
1. Control Panel → Programs → Uninstall a program
2. Select "Advanced Network Packet Sniffer" → Uninstall

**Clean Removal**: User data in AppData is also removed automatically.

## 📖 Additional Help

### Documentation
- **README.md**: Comprehensive user guide
- **examples.md**: Practical usage scenarios  
- **PRODUCTION_GUIDE.md**: Enterprise deployment guide

### Getting Support
1. Use built-in help: `packet_sniffer.exe --help`
2. Enable verbose output: `--verbose` flag
3. Check log files in `%APPDATA%\packet_sniffer\logs\`
4. Review Windows Event Viewer for system errors

### Pro Tips
- Start with dashboard mode for best experience
- Use `--list-interfaces` to see exact interface names
- Interface names are case-sensitive
- Always run as Administrator for full functionality
- Check firewall settings if experiencing connection issues

This tool transforms complex network analysis into an accessible, user-friendly experience while maintaining professional-grade capabilities!