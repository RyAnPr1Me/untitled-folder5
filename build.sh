#!/bin/bash

# Advanced Network Packet Sniffer Build Script
# This script builds the application for multiple platforms and creates installers

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Advanced Network Packet Sniffer Build Script${NC}"
echo "=================================================="

# Function to print status
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

# Check if we're on Linux (for cross-compilation setup)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    print_status "Detected Linux environment"
    
    # Install Windows target if not already installed
    if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
        print_status "Installing Windows cross-compilation target..."
        rustup target add x86_64-pc-windows-gnu
    fi
    
    # Check if mingw-w64 is installed
    if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        print_warning "mingw-w64 not found. Installing for Windows cross-compilation..."
        # Try different package managers
        if command -v apt-get &> /dev/null; then
            sudo apt-get update
            sudo apt-get install -y mingw-w64 gcc-mingw-w64-x86-64
        elif command -v yum &> /dev/null; then
            sudo yum install -y mingw64-gcc
        elif command -v pacman &> /dev/null; then
            sudo pacman -S mingw-w64-gcc
        else
            print_error "Could not install mingw-w64. Please install manually."
            exit 1
        fi
    fi
fi

# Create build directories
mkdir -p dist
mkdir -p target

print_status "Building for Linux (release)..."
cargo build --release
print_status "Linux build completed"

# Copy Linux binary
cp target/release/packet_sniffer dist/packet_sniffer_linux

# Build for Windows if on Linux
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    print_status "Cross-compiling for Windows..."
    
    # Set Windows-specific environment variables
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_ALL_STATIC=1
    
    # Build for Windows
    if cargo build --release --target x86_64-pc-windows-gnu; then
        print_status "Windows cross-compilation completed"
        cp target/x86_64-pc-windows-gnu/release/packet_sniffer.exe dist/
    else
        print_warning "Windows cross-compilation failed. Skipping Windows build."
    fi
fi

# Create configuration directory structure
mkdir -p dist/config
mkdir -p dist/docs

# Generate default configuration
print_status "Generating default configuration..."
if [[ -f "dist/packet_sniffer_linux" ]]; then
    ./dist/packet_sniffer_linux --generate-config --config dist/config/default_config.json || true
elif [[ -f "target/release/packet_sniffer" ]]; then
    ./target/release/packet_sniffer --generate-config --config dist/config/default_config.json || true
fi

# Copy documentation
cp README.md dist/
cp examples.md dist/
cp LICENSE dist/
cp INSTALL_README.txt dist/

# Create documentation directory
mkdir -p dist/docs
echo "# Advanced Network Packet Sniffer Documentation

This directory contains additional documentation files.

## Quick Start
1. Run with administrator privileges
2. Use --list-interfaces to see available network interfaces  
3. Use --interface <name> to start capturing
4. Use --dashboard for interactive mode

## Command Reference
See README.md for complete command reference and examples.

## Configuration
The application uses a JSON configuration file for customization.
Use --generate-config to create a default configuration file.
" > dist/docs/README.md

# Create version info
echo "1.0.0" > dist/VERSION
echo "$(date -u +"%Y-%m-%d %H:%M:%S UTC")" > dist/BUILD_DATE

# Create simple installer script for Linux
cat > dist/install_linux.sh << 'EOF'
#!/bin/bash

echo "🌐 Installing Advanced Network Packet Sniffer..."

# Check for root privileges
if [[ $EUID -eq 0 ]]; then
    INSTALL_DIR="/usr/local/bin"
    CONFIG_DIR="/etc/packet_sniffer"
else
    INSTALL_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/packet_sniffer"
    mkdir -p "$HOME/.local/bin"
fi

# Copy binary
cp packet_sniffer_linux "$INSTALL_DIR/packet_sniffer"
chmod +x "$INSTALL_DIR/packet_sniffer"

# Create config directory
mkdir -p "$CONFIG_DIR"
if [[ -f "config/default_config.json" ]]; then
    cp config/default_config.json "$CONFIG_DIR/"
fi

echo "✅ Installation completed!"
echo "📍 Binary installed to: $INSTALL_DIR/packet_sniffer"
echo "📍 Configuration directory: $CONFIG_DIR"
echo ""
echo "🚀 Quick start:"
echo "   sudo packet_sniffer --list-interfaces"
echo "   sudo packet_sniffer --interface eth0 --dashboard"
echo ""
echo "💡 Add $INSTALL_DIR to your PATH if not already included"
EOF

chmod +x dist/install_linux.sh

# Create package information
cat > dist/PACKAGE_INFO.txt << EOF
Advanced Network Packet Sniffer v1.0.0
Built on: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Build system: $(uname -a)
Rust version: $(rustc --version)

Package Contents:
- packet_sniffer_linux: Linux executable
- packet_sniffer.exe: Windows executable (if available)
- README.md: User documentation
- examples.md: Usage examples
- LICENSE: Software license
- INSTALL_README.txt: Installation instructions
- config/: Default configuration files
- docs/: Additional documentation
- install_linux.sh: Linux installation script
- installer.iss: Windows Inno Setup script

Installation:
Linux: Run ./install_linux.sh
Windows: Use Inno Setup with installer.iss to create Windows installer

Requirements:
- Administrator/root privileges for packet capture
- Active network interface
- Minimum 4GB RAM recommended
EOF

print_status "Build completed successfully!"
echo ""
echo "📦 Package contents in dist/ directory:"
ls -la dist/
echo ""
print_status "Linux installation: cd dist && ./install_linux.sh"
if [[ -f "dist/packet_sniffer.exe" ]]; then
    print_status "Windows installer: Use Inno Setup with installer.iss"
fi
echo ""
print_warning "Remember: This application requires administrator privileges to capture packets!"