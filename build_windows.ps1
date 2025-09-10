# Advanced Network Packet Sniffer - Windows Build Script (PowerShell)
# This script automates the complete build process for Windows including Inno Setup compilation

param(
    [switch]$SkipInstaller,
    [switch]$Verbose,
    [string]$Configuration = "release"
)

# Enable verbose output if requested
if ($Verbose) {
    $VerbosePreference = "Continue"
}

Write-Host ""
Write-Host "========================================"
Write-Host "Advanced Network Packet Sniffer"
Write-Host "Windows Build and Installer Creation"
Write-Host "========================================"
Write-Host ""

# Function to check if command exists
function Test-Command($cmdname) {
    return [bool](Get-Command -Name $cmdname -ErrorAction SilentlyContinue)
}

# Function to create directory if it doesn't exist
function New-DirectoryIfNotExists($path) {
    if (!(Test-Path -Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

try {
    # Step 1: Verify project directory
    Write-Host "[1/9] Verifying project structure..." -ForegroundColor Green
    if (!(Test-Path "Cargo.toml")) {
        throw "Cargo.toml not found. Please run this script from the project root directory."
    }
    Write-Host "✓ Project structure verified" -ForegroundColor Green

    # Step 2: Clean previous builds
    Write-Host ""
    Write-Host "[2/9] Cleaning previous builds..." -ForegroundColor Green
    if (Test-Path "target") {
        Remove-Item -Path "target" -Recurse -Force
    }
    if (Test-Path "dist") {
        Remove-Item -Path "dist" -Recurse -Force
    }
    New-DirectoryIfNotExists "dist"
    Write-Host "✓ Build directories cleaned" -ForegroundColor Green

    # Step 3: Check Rust installation
    Write-Host ""
    Write-Host "[3/9] Checking Rust installation..." -ForegroundColor Green
    if (!(Test-Command "cargo")) {
        throw "Rust/Cargo is not installed or not in PATH. Please install from https://rustup.rs/"
    }
    $rustVersion = & cargo --version
    Write-Host "✓ $rustVersion" -ForegroundColor Green

    # Step 4: Update Rust toolchain
    Write-Host ""
    Write-Host "[4/9] Updating Rust toolchain..." -ForegroundColor Green
    & rustup update stable
    & rustup default stable
    Write-Host "✓ Rust toolchain updated" -ForegroundColor Green

    # Step 5: Check for Windows dependencies
    Write-Host ""
    Write-Host "[5/9] Checking Windows dependencies..." -ForegroundColor Green
    
    # Check for Visual Studio Build Tools
    $msbuildPaths = @(
        "${env:ProgramFiles}\Microsoft Visual Studio\2022\*\MSBuild\Current\Bin\MSBuild.exe",
        "${env:ProgramFiles}\Microsoft Visual Studio\2019\*\MSBuild\Current\Bin\MSBuild.exe",
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2019\*\MSBuild\Current\Bin\MSBuild.exe"
    )
    
    $msbuildFound = $false
    foreach ($path in $msbuildPaths) {
        if (Get-ChildItem $path -ErrorAction SilentlyContinue) {
            $msbuildFound = $true
            break
        }
    }
    
    if (!$msbuildFound) {
        Write-Host "⚠ Warning: Visual Studio Build Tools not detected" -ForegroundColor Yellow
        Write-Host "  Build may fail. Install Visual Studio Build Tools if needed." -ForegroundColor Yellow
    } else {
        Write-Host "✓ Visual Studio Build Tools detected" -ForegroundColor Green
    }

    # Step 6: Build release version
    Write-Host ""
    Write-Host "[6/9] Building $Configuration version (this may take several minutes)..." -ForegroundColor Green
    Write-Host "This step downloads dependencies and compiles the application..." -ForegroundColor Gray
    
    $buildArgs = @("build", "--$Configuration")
    if ($Verbose) {
        $buildArgs += "--verbose"
    }
    
    $buildProcess = Start-Process -FilePath "cargo" -ArgumentList $buildArgs -Wait -PassThru -NoNewWindow
    if ($buildProcess.ExitCode -ne 0) {
        throw "Build failed with exit code $($buildProcess.ExitCode)"
    }
    Write-Host "✓ Release build completed successfully" -ForegroundColor Green

    # Step 7: Create distribution structure
    Write-Host ""
    Write-Host "[7/9] Creating distribution structure..." -ForegroundColor Green
    
    $distDirs = @("config", "docs", "logs", "exports")
    foreach ($dir in $distDirs) {
        New-DirectoryIfNotExists "dist\$dir"
    }
    Write-Host "✓ Directory structure created" -ForegroundColor Green

    # Step 8: Copy distribution files
    Write-Host ""
    Write-Host "[8/9] Preparing distribution files..." -ForegroundColor Green
    
    # Copy main executable
    $exePath = "target\$Configuration\packet_sniffer.exe"
    if (!(Test-Path $exePath)) {
        throw "Executable not found at $exePath"
    }
    Copy-Item $exePath "dist\" -Force
    
    # Copy documentation files
    $docFiles = @(
        "README.md", "LICENSE", "examples.md", 
        "PRODUCTION_GUIDE.md", "WINDOWS_BUILD.md", "RELEASE_NOTES.md"
    )
    
    foreach ($file in $docFiles) {
        if (Test-Path $file) {
            Copy-Item $file "dist\" -Force
        }
    }
    
    # Copy optional files
    $optionalFiles = @("packet_sniffer.exe.manifest", "icon.ico", "INSTALL_README.txt")
    foreach ($file in $optionalFiles) {
        if (Test-Path $file) {
            Copy-Item $file "dist\" -Force
        }
    }
    
    Write-Host "✓ Distribution files copied" -ForegroundColor Green

    # Generate default configuration
    Write-Host ""
    Write-Host "Generating default configuration..." -ForegroundColor Gray
    try {
        $configPath = "dist\config\default_config.json"
        & "dist\packet_sniffer.exe" --generate-config --config $configPath
        Write-Host "✓ Default configuration generated" -ForegroundColor Green
    }
    catch {
        Write-Host "⚠ Warning: Could not generate default configuration" -ForegroundColor Yellow
    }

    # Step 9: Create installer with Inno Setup
    if (!$SkipInstaller) {
        Write-Host ""
        Write-Host "[9/9] Creating Windows installer..." -ForegroundColor Green
        
        if (Test-Command "iscc") {
            Write-Host "Found Inno Setup Compiler, creating installer..." -ForegroundColor Gray
            $installerProcess = Start-Process -FilePath "iscc" -ArgumentList "installer.iss" -Wait -PassThru -NoNewWindow
            
            if ($installerProcess.ExitCode -eq 0) {
                Write-Host "✓ Windows installer created successfully!" -ForegroundColor Green
                $installerPath = "dist\PacketSnifferSetup-1.0.0.exe"
                if (Test-Path $installerPath) {
                    $installerSize = [math]::Round((Get-Item $installerPath).Length / 1MB, 2)
                    Write-Host "  Installer: $installerPath ($installerSize MB)" -ForegroundColor Gray
                }
            } else {
                Write-Host "⚠ Warning: Installer creation failed" -ForegroundColor Yellow
            }
        } else {
            Write-Host "⚠ Warning: Inno Setup Compiler (iscc) not found in PATH" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "To create the installer:" -ForegroundColor Gray
            Write-Host "1. Install Inno Setup from https://jrsoftware.org/isinfo.php" -ForegroundColor Gray
            Write-Host "2. Add Inno Setup to your PATH, or" -ForegroundColor Gray
            Write-Host "3. Run: iscc installer.iss" -ForegroundColor Gray
        }
    } else {
        Write-Host ""
        Write-Host "[9/9] Skipping installer creation (--SkipInstaller flag used)" -ForegroundColor Yellow
    }

    # Success summary
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "Build completed successfully!" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Output files:" -ForegroundColor White
    Write-Host "  • Executable: dist\packet_sniffer.exe" -ForegroundColor Gray
    Write-Host "  • Documentation: dist\*.md" -ForegroundColor Gray
    Write-Host "  • Configuration: dist\config\default_config.json" -ForegroundColor Gray
    
    if (Test-Path "dist\PacketSnifferSetup-1.0.0.exe") {
        Write-Host "  • Installer: dist\PacketSnifferSetup-1.0.0.exe" -ForegroundColor Gray
    }
    
    Write-Host ""
    Write-Host "The application is ready for distribution!" -ForegroundColor Green
    Write-Host ""
    
    # Get executable info
    $exeInfo = Get-Item "dist\packet_sniffer.exe"
    Write-Host "Executable size: $([math]::Round($exeInfo.Length / 1MB, 2)) MB" -ForegroundColor Gray
    Write-Host "Build time: $(Get-Date)" -ForegroundColor Gray

} catch {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Red
    Write-Host "Build failed!" -ForegroundColor Red
    Write-Host "========================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    
    if ($_.Exception.Message -like "*Cargo*" -or $_.Exception.Message -like "*rust*") {
        Write-Host "Troubleshooting tips:" -ForegroundColor Yellow
        Write-Host "1. Install Rust from: https://rustup.rs/" -ForegroundColor Gray
        Write-Host "2. Restart your terminal/PowerShell" -ForegroundColor Gray
        Write-Host "3. Run: rustup update" -ForegroundColor Gray
    }
    
    exit 1
}

# Optional: Open dist folder in Explorer
$openDist = Read-Host "Open distribution folder in Explorer? (y/N)"
if ($openDist -eq "y" -or $openDist -eq "Y") {
    Start-Process explorer.exe -ArgumentList (Resolve-Path "dist").Path
}