@echo off
REM Advanced Network Packet Sniffer - Windows Build Script
REM This script automates the complete build process for Windows including Inno Setup compilation

echo.
echo ========================================
echo Advanced Network Packet Sniffer
echo Windows Build and Installer Creation
echo ========================================
echo.

REM Check if running in correct directory
if not exist "Cargo.toml" (
    echo ERROR: Cargo.toml not found. Please run this script from the project root directory.
    pause
    exit /b 1
)

REM Step 1: Clean previous builds
echo [1/8] Cleaning previous builds...
if exist "target" rmdir /s /q "target"
if exist "dist" rmdir /s /q "dist"
mkdir dist
echo ✓ Build directories cleaned

REM Step 2: Check Rust installation
echo.
echo [2/8] Checking Rust installation...
rust --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust is not installed or not in PATH.
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)
echo ✓ Rust installation verified

REM Step 3: Update Rust and dependencies
echo.
echo [3/8] Updating Rust toolchain...
rustup update stable
rustup default stable
echo ✓ Rust toolchain updated

REM Step 4: Build release version
echo.
echo [4/8] Building release version (this may take several minutes)...
cargo build --release
if errorlevel 1 (
    echo ERROR: Build failed. Please check the error messages above.
    pause
    exit /b 1
)
echo ✓ Release build completed successfully

REM Step 5: Create directory structure
echo.
echo [5/8] Creating distribution directory structure...
mkdir dist\config 2>nul
mkdir dist\docs 2>nul
mkdir dist\logs 2>nul
mkdir dist\exports 2>nul
echo ✓ Directory structure created

REM Step 6: Copy files for distribution
echo.
echo [6/8] Copying distribution files...

REM Copy main executable
copy "target\release\packet_sniffer.exe" "dist\" >nul
if errorlevel 1 (
    echo ERROR: Failed to copy main executable
    pause
    exit /b 1
)

REM Copy documentation
copy "README.md" "dist\" >nul
copy "LICENSE" "dist\" >nul
copy "examples.md" "dist\" >nul
copy "PRODUCTION_GUIDE.md" "dist\" >nul
copy "WINDOWS_BUILD.md" "dist\" >nul
copy "RELEASE_NOTES.md" "dist\" >nul
copy "INSTALL_README.txt" "dist\" >nul

REM Copy configuration files
if exist "packet_sniffer.exe.manifest" copy "packet_sniffer.exe.manifest" "dist\" >nul
if exist "icon.ico" copy "icon.ico" "dist\" >nul

echo ✓ Distribution files copied

REM Step 7: Generate default configuration
echo.
echo [7/8] Generating default configuration...
"target\release\packet_sniffer.exe" --generate-config --config "dist\config\default_config.json"
if errorlevel 1 (
    echo WARNING: Failed to generate default configuration
) else (
    echo ✓ Default configuration generated
)

REM Step 8: Check for Inno Setup and create installer
echo.
echo [8/8] Creating Windows installer with Inno Setup...

REM Check if Inno Setup is installed
where iscc >nul 2>&1
if errorlevel 1 (
    echo WARNING: Inno Setup Compiler (iscc) not found in PATH.
    echo.
    echo To create the installer:
    echo 1. Install Inno Setup from https://jrsoftware.org/isinfo.php
    echo 2. Add Inno Setup to your PATH, or
    echo 3. Run the following command manually:
    echo    "C:\Program Files (x86)\Inno Setup 6\iscc.exe" installer.iss
    echo.
    echo The build files are ready in the 'dist' directory.
) else (
    echo Found Inno Setup Compiler, creating installer...
    iscc installer.iss
    if errorlevel 1 (
        echo ERROR: Installer creation failed
        pause
        exit /b 1
    ) else (
        echo ✓ Windows installer created successfully!
        echo.
        echo Installer location: dist\PacketSnifferSetup-1.0.0.exe
    )
)

echo.
echo ========================================
echo Build completed successfully!
echo ========================================
echo.
echo Output files:
echo   • Executable: dist\packet_sniffer.exe
echo   • Documentation: dist\*.md
echo   • Configuration: dist\config\default_config.json
if exist "dist\PacketSnifferSetup-1.0.0.exe" (
    echo   • Installer: dist\PacketSnifferSetup-1.0.0.exe
)
echo.
echo The application is ready for distribution!
echo.

pause