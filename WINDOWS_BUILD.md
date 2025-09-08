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

1. **Install Inno Setup**
   - Download from https://jrsoftware.org/isinfo.php
   - Install with full components

2. **Prepare installer files**
   - Ensure all files are in the correct locations as specified in installer.iss
   - Verify packet_sniffer.exe is in target\release\
   - Create or replace icon.ico with a proper icon file

3. **Compile installer**
   - Open Inno Setup Compiler
   - Open installer.iss
   - Click Build > Compile
   - The installer will be created in the dist\ directory

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