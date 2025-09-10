# Windows Build Instructions

## Prerequisites

1. **Install Rust for Windows**
   - Download from https://rustup.rs/
   - Run the installer and follow the setup instructions

2. **Install Build Dependencies**
   - Visual Studio Build Tools 2019 or later
   - Windows SDK
   - Git for Windows

3. **Install WinPcap/Npcap Development Libraries**
   - Download Npcap SDK from https://npcap.com/
   - Extract to a known location (e.g., C:\npcap-sdk)
   - Set environment variable: `LIB=C:\npcap-sdk\Lib\x64;%LIB%`

## Building on Windows

1. **Clone the repository**
   ```cmd
   git clone <repository-url>
   cd packet_sniffer
   ```

2. **Set up environment**
   ```cmd
   # Add Npcap SDK to library path
   set LIB=C:\npcap-sdk\Lib\x64;%LIB%
   ```

3. **Build release version**
   ```cmd
   cargo build --release
   ```

4. **Copy necessary files**
   ```cmd
   # Create distribution directory
   mkdir dist
   copy target\release\packet_sniffer.exe dist\
   copy README.md dist\
   copy LICENSE dist\
   copy examples.md dist\
   copy INSTALL_README.txt dist\
   copy packet_sniffer.exe.manifest dist\
   
   # Create config directory
   mkdir dist\config
   packet_sniffer.exe --generate-config --config dist\config\default_config.json
   ```

## Creating Windows Installer

### Automated Build Process (Recommended)

#### Method 1: PowerShell Script
```powershell
# Full build with installer
.\build_windows.ps1

# Build with verbose output
.\build_windows.ps1 -Verbose

# Build without installer creation
.\build_windows.ps1 -SkipInstaller
```

#### Method 2: Batch Script
```cmd
# Run automated build
build_windows.bat
```

Both scripts will:
1. Clean previous builds
2. Verify Rust installation
3. Build release version
4. Create distribution structure
5. Copy all necessary files
6. Generate default configuration
7. Compile Inno Setup installer (if available)

### Manual Inno Setup Compilation

1. **Install Inno Setup**
   - Download from https://jrsoftware.org/isinfo.php
   - Install with full components

2. **Prepare build files**
   ```cmd
   # Build the application
   cargo build --release
   
   # Create distribution structure
   mkdir dist config docs
   
   # Generate configuration
   target\release\packet_sniffer.exe --generate-config --config config\default_config.json
   ```

3. **Compile installer**
   
   **Option A: GUI Method**
   - Open Inno Setup
   - File → Open → `installer.iss`
   - Build → Compile
   
   **Option B: Command Line**
   ```cmd
   "C:\Program Files (x86)\Inno Setup 6\iscc.exe" installer.iss
   ```
   
   **Option C: Context Menu**
   - Right-click `installer.iss`
   - Select "Compile"

4. **Output**
   - Installer created: `dist\PacketSnifferSetup-1.0.0.exe`
   - Size: ~15-25 MB (including dependencies)
   - Features: Professional installer with dependency management

### Installer Features

The Inno Setup installer includes:

#### Professional Installation Experience
- Modern wizard-style interface
- Multi-language support (English, Spanish, French, German, Japanese)
- Component selection (Full, Compact, Custom)
- Optional features (desktop shortcuts, PATH integration, firewall config)

#### Dependency Management
- **Npcap Driver Detection**: Automatically checks and prompts for installation
- **Visual C++ Redistributable**: Silent installation if needed
- **Administrator Privileges**: Validates and requires elevation

#### Advanced Configuration
- **System Integration**: Optional PATH addition and Windows service installation
- **Security**: Windows Firewall exception configuration
- **Documentation**: Complete help system with examples and guides
- **Uninstaller**: Clean removal with registry cleanup

#### Installation Locations
```
Program Files:
  C:\Program Files\Advanced Network Packet Sniffer\
  ├── packet_sniffer.exe
  ├── config\default_config.json
  ├── docs\
  └── README.md, examples.md, etc.

User Data:
  %APPDATA%\packet_sniffer\
  ├── exports\
  └── logs\
```

## Prerequisites for Users

### Windows System Requirements
- Windows 10 or later (64-bit)
- Administrator privileges
- Npcap driver (automatically installed with Wireshark or can be downloaded separately)

### Npcap Installation
If Npcap is not already installed:
1. Download from https://npcap.com/
2. Run the installer as Administrator
3. Enable "Install Npcap in WinPcap API-compatible Mode" if needed

## Troubleshooting Build Issues

### "cannot find -lPacket" Error
- Ensure Npcap SDK is properly installed
- Verify LIB environment variable includes Npcap library path
- Check that x64 libraries are being used for 64-bit builds

### Missing Visual Studio Build Tools
- Install Visual Studio Build Tools
- Or install full Visual Studio Community edition
- Ensure C++ build tools are included

### Linker Errors
- Update Rust: `rustup update`
- Clear build cache: `cargo clean`
- Rebuild: `cargo build --release`

## Distribution Notes

The compiled executable requires:
1. Windows 10/11 64-bit
2. Npcap driver installed
3. Administrator privileges to run
4. Network interface present and active

For enterprise distribution, consider:
- Code signing the executable
- Creating an MSI installer instead of Inno Setup
- Including Npcap installer in the distribution package
- Group Policy deployment for large organizations