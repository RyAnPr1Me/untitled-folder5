# Inno Setup Compilation Guide

This guide provides step-by-step instructions for compiling the Advanced Network Packet Sniffer using Inno Setup to create a professional Windows installer.

## 📋 Prerequisites

### Required Software

1. **Inno Setup 6.x** (Latest version recommended)
   - Download from: https://jrsoftware.org/isinfo.php
   - Install with full components including:
     - Inno Setup Compiler
     - Inno Setup QuickStart Pack
     - Documentation and Examples

2. **Rust Development Environment**
   - Rust toolchain (latest stable)
   - Visual Studio Build Tools 2019/2022
   - Windows SDK

3. **Optional Tools**
   - Code signing certificate (for production distribution)
   - Inno Setup Script Studio (enhanced IDE)

### Project Structure Verification

Ensure your project has the following structure:
```
packet_sniffer/
├── Cargo.toml
├── src/
├── target/release/packet_sniffer.exe  (after building)
├── installer.iss                      (Inno Setup script)
├── icon.ico                          (Application icon)
├── INSTALL_README.txt                (Installation guide)
├── config/default_config.json        (Default configuration)
├── README.md                         (Documentation)
├── examples.md
├── PRODUCTION_GUIDE.md
├── WINDOWS_BUILD.md
├── RELEASE_NOTES.md
└── LICENSE
```

## 🛠️ Step-by-Step Compilation Process

### Step 1: Build the Rust Application

First, ensure the application builds successfully:

```cmd
# Clean previous builds
cargo clean

# Build release version with optimizations
cargo build --release
```

The executable will be created at: `target\release\packet_sniffer.exe`

### Step 2: Prepare Distribution Files

Create the necessary directory structure:

```cmd
# Create distribution directories
mkdir dist
mkdir config
mkdir docs

# Generate default configuration
target\release\packet_sniffer.exe --generate-config --config config\default_config.json
```

### Step 3: Using Automated Build Scripts

#### Option A: Batch Script (Windows CMD)
```cmd
# Run the automated build script
build_windows.bat
```

#### Option B: PowerShell Script (Recommended)
```powershell
# Run with full options
.\build_windows.ps1

# Or with specific options
.\build_windows.ps1 -Verbose -SkipInstaller
```

### Step 4: Manual Inno Setup Compilation

If you prefer manual compilation:

#### Method 1: Inno Setup IDE (Recommended)
1. **Open Inno Setup**
2. **File** → **Open** → Select `installer.iss`
3. **Build** → **Compile**
4. Monitor the compilation process in the output window

#### Method 2: Command Line
```cmd
# Navigate to project directory
cd /path/to/packet_sniffer

# Compile using command line
"C:\Program Files (x86)\Inno Setup 6\iscc.exe" installer.iss
```

#### Method 3: Right-Click Context Menu
1. Right-click `installer.iss` in Windows Explorer
2. Select **"Compile"** from the context menu

## 📦 Installer Configuration Details

### Key Configuration Settings

The `installer.iss` file includes:

#### Application Information
```pascal
#define MyAppName "Advanced Network Packet Sniffer"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Packet Sniffer Team"
#define MyAppExeName "packet_sniffer.exe"
```

#### Installation Options
- **Target Architecture**: x64 only
- **Privileges Required**: Administrator
- **Compression**: LZMA2 Ultra
- **Installer Type**: Modern wizard style

#### Custom Features
- Multi-language support (English, Spanish, French, German, Japanese)
- Dependency checking (Npcap, Visual C++ Redistributable)
- Optional PATH integration
- Windows Firewall configuration
- Service installation support

### File Inclusion Rules

The installer includes:
- Main executable with digital signature verification
- Configuration files and templates
- Complete documentation suite
- Application icon and manifest
- Runtime dependencies (if needed)

## 🔧 Advanced Configuration Options

### Code Signing (Production)

For commercial distribution, add code signing:

```pascal
[Files]
Source: "target\release\packet_sniffer.exe"; DestDir: "{app}"; Flags: ignoreversion signonce
```

Configure signing in Inno Setup:
1. **Tools** → **Configure Sign Tools**
2. Add your code signing certificate
3. Configure signing parameters

### Custom Installation Types

The installer supports three installation types:

#### Full Installation (Recommended)
- Complete application with all features
- Full documentation suite
- Configuration templates
- Additional tools and scripts

#### Compact Installation
- Core application only
- Essential documentation
- Minimal disk space usage

#### Custom Installation
- User selects specific components
- Flexible configuration options
- Advanced user control

### Dependency Management

The installer automatically handles:

#### Npcap Driver
- Detection of existing installation
- Prompt for download if not found
- Compatibility mode configuration

#### Visual C++ Redistributable
- Automatic detection
- Silent installation if needed
- Version compatibility checking

## 📊 Compilation Output

### Successful Compilation

Upon successful compilation, you'll get:

```
PacketSnifferSetup-1.0.0.exe
├── Size: ~15-25 MB (depending on dependencies)
├── Features: Self-extracting installer
├── Compression: LZMA2 Ultra
└── Digital Signature: Optional (if configured)
```

### Installation Package Contents

The installer will deploy:
- Application executable (2-3 MB)
- Configuration files
- Documentation (PDF/HTML)
- Start Menu shortcuts
- Desktop shortcuts (optional)
- Uninstaller

## 🐛 Troubleshooting Compilation Issues

### Common Problems and Solutions

#### "File not found" Errors
```
Error: Cannot open file "target\release\packet_sniffer.exe"
```
**Solution**: Build the Rust application first with `cargo build --release`

#### Permission Denied
```
Error: Cannot write to output directory
```
**Solution**: 
- Run Inno Setup as Administrator
- Check folder permissions
- Ensure output directory is writable

#### Missing Dependencies
```
Warning: Npcap not found
```
**Solution**: This is expected - the installer will handle Npcap installation

#### Large Installer Size
```
Warning: Installer larger than expected
```
**Solution**: 
- Check if debug symbols are included
- Verify compression settings
- Review included files list

### Advanced Debugging

#### Enable Verbose Output
Add to installer.iss:
```pascal
[Setup]
OutputMessagesFilename=Setup-Messages.txt
```

#### Test Installation
```cmd
# Silent installation for testing
PacketSnifferSetup-1.0.0.exe /SILENT /LOG=install.log

# Unattended installation
PacketSnifferSetup-1.0.0.exe /VERYSILENT /NORESTART
```

## 🚀 Distribution Preparation

### Pre-Distribution Checklist

- [ ] Application builds and runs correctly
- [ ] All dependencies are identified
- [ ] Documentation is up-to-date
- [ ] Installer compiles without errors
- [ ] Test installation on clean Windows system
- [ ] Verify uninstaller works correctly
- [ ] Check that all shortcuts function properly
- [ ] Test with different user privilege levels

### Testing the Installer

#### Virtual Machine Testing
Test the installer on:
- Clean Windows 10 installation
- Clean Windows 11 installation
- System without Visual C++ Redistributable
- System without Npcap driver

#### Installation Scenarios
- Standard user account (should prompt for elevation)
- Administrator account
- Domain-joined computer
- Offline computer (verify dependencies)

### Final Package Verification

Verify the installer:
1. **Runs without errors**
2. **Installs all components correctly**
3. **Creates proper Start Menu entries**
4. **Application launches successfully**
5. **Uninstaller removes all files**
6. **No registry orphans remain**

## 📋 Distribution Notes

### System Requirements Documentation
Ensure your distribution includes:
- Minimum Windows version (Windows 10)
- Required privileges (Administrator)
- Network interface requirements
- Dependency information (Npcap)

### User Experience Optimization
- Clear installation instructions
- Helpful error messages
- Comprehensive documentation
- Professional installer appearance
- Intuitive user interface

The resulting installer provides enterprise-grade installation experience while maintaining ease of use for end users!