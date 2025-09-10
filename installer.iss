; Inno Setup Script for Advanced Network Packet Sniffer
; This script creates a professional Windows installer for the packet sniffer application
; with enterprise features, dependency management, and comprehensive user experience

#define MyAppName "Advanced Network Packet Sniffer"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Packet Sniffer Team"
#define MyAppURL "https://github.com/RyAnPr1Me/untitled-folder5"
#define MyAppExeName "packet_sniffer.exe"
#define MyAppDescription "Enterprise-grade Network Packet Sniffer with Advanced Dashboard GUI and Real-time Threat Detection"
#define MyAppId "C7F2E5A1-8B4D-4A3F-9E2C-5D6F7A8B9C0D"

[Setup]
; NOTE: The value of AppId uniquely identifies this application.
AppId={{{#MyAppId}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=LICENSE
InfoBeforeFile=INSTALL_README.txt
InfoAfterFile=PRODUCTION_GUIDE.md
OutputDir=dist
OutputBaseFilename=PacketSnifferSetup-{#MyAppVersion}
SetupIconFile=icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=6.1sp1
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\icon.ico
VersionInfoVersion={#MyAppVersion}
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription={#MyAppDescription}
VersionInfoCopyright=Copyright (C) 2024 {#MyAppPublisher}
VersionInfoProductName={#MyAppName}
VersionInfoProductVersion={#MyAppVersion}
AppMutex=PacketSnifferMutex

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1
Name: "addtopath"; Description: "Add {#MyAppName} to system PATH"; GroupDescription: "System Integration"; Flags: unchecked
Name: "startupservice"; Description: "Install as Windows service (advanced users)"; GroupDescription: "Service Configuration"; Flags: unchecked
Name: "firewall"; Description: "Configure Windows Firewall exceptions"; GroupDescription: "Security Configuration"; Flags: unchecked

[Files]
; Main executable and essential files
Source: "target\release\packet_sniffer.exe"; DestDir: "{app}"; Flags: ignoreversion signonce
Source: "packet_sniffer.exe.manifest"; DestDir: "{app}"; Flags: ignoreversion
Source: "icon.ico"; DestDir: "{app}"; Flags: ignoreversion

; Build scripts for developers
Source: "build_windows.bat"; DestDir: "{app}"; Flags: ignoreversion; Components: tools
Source: "build_windows.ps1"; DestDir: "{app}"; Flags: ignoreversion; Components: tools
Source: "build.sh"; DestDir: "{app}"; Flags: ignoreversion; Components: tools

; Documentation files
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion; Components: main
Source: "examples.md"; DestDir: "{app}"; Flags: ignoreversion; Components: docs
Source: "PRODUCTION_GUIDE.md"; DestDir: "{app}"; Flags: ignoreversion; Components: docs
Source: "WINDOWS_BUILD.md"; DestDir: "{app}"; Flags: ignoreversion; Components: docs
Source: "INNO_SETUP_GUIDE.md"; DestDir: "{app}"; Flags: ignoreversion; Components: docs
Source: "RELEASE_NOTES.md"; DestDir: "{app}"; Flags: ignoreversion; Components: docs
Source: "LICENSE"; DestDir: "{app}"; Flags: ignoreversion; Components: main

; Configuration files
Source: "config\default_config.json"; DestDir: "{app}\config"; Flags: ignoreversion; Components: config
Source: "config\*"; DestDir: "{app}\config"; Flags: ignoreversion recursesubdirs createallsubdirs; Excludes: "*.tmp,*.log"; Components: config

; Documentation directory with comprehensive guides
Source: "docs\*"; DestDir: "{app}\docs"; Flags: ignoreversion recursesubdirs createallsubdirs; Components: docs

; Installer and development files for reference
Source: "installer.iss"; DestDir: "{app}"; Flags: ignoreversion; Components: tools
Source: "Cargo.toml"; DestDir: "{app}"; Flags: ignoreversion; Components: tools
Source: "Cargo-windows.toml"; DestDir: "{app}"; Flags: ignoreversion; Components: tools

; Runtime dependencies (if any Visual C++ redistributables are needed)
; Source: "vcredist_x64.exe"; DestDir: "{tmp}"; Flags: deleteafterinstall; Check: VCRedistNeedsInstall

[Icons]
; Start Menu shortcuts
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; Comment: "{#MyAppDescription}"; IconFilename: "{app}\icon.ico"
Name: "{group}\{#MyAppName} Dashboard"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--dashboard"; Comment: "Launch advanced interactive dashboard with real-time analytics"; IconFilename: "{app}\icon.ico"; WorkingDir: "{app}"
Name: "{group}\List Network Interfaces"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--list-interfaces"; Comment: "Show available network interfaces with status"; IconFilename: "{app}\icon.ico"
Name: "{group}\Generate Configuration"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--generate-config"; Comment: "Generate default configuration file"; IconFilename: "{app}\icon.ico"

; Documentation shortcuts
Name: "{group}\Documentation\User Guide"; Filename: "{app}\README.md"; Comment: "Read the comprehensive user guide"
Name: "{group}\Documentation\Examples & Tutorials"; Filename: "{app}\examples.md"; Comment: "View usage examples and tutorials"
Name: "{group}\Documentation\Production Guide"; Filename: "{app}\PRODUCTION_GUIDE.md"; Comment: "Enterprise deployment and scaling guide"
Name: "{group}\Documentation\Windows Build Guide"; Filename: "{app}\WINDOWS_BUILD.md"; Comment: "Windows compilation instructions"
Name: "{group}\Documentation\Inno Setup Guide"; Filename: "{app}\INNO_SETUP_GUIDE.md"; Comment: "Detailed installer creation guide"
Name: "{group}\Documentation\Release Notes"; Filename: "{app}\RELEASE_NOTES.md"; Comment: "Version history and changelog"
Name: "{group}\Documentation\Documentation Index"; Filename: "{app}\docs\README.md"; Comment: "Complete documentation index"

; Uninstaller
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"

; Desktop shortcuts
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; Comment: "{#MyAppDescription}"; IconFilename: "{app}\icon.ico"; WorkingDir: "{app}"
Name: "{autodesktop}\{#MyAppName} Dashboard"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--dashboard"; Tasks: desktopicon; Comment: "Launch dashboard mode"; IconFilename: "{app}\icon.ico"; WorkingDir: "{app}"

; Quick Launch (legacy)
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: quicklaunchicon; IconFilename: "{app}\icon.ico"

[Run]
; Post-installation tasks
Filename: "{app}\{#MyAppExeName}"; Parameters: "--generate-config --config ""{app}\config\default_config.json"""; WorkingDir: "{app}"; Flags: runhidden; Description: "Generate default configuration"
Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; Description: "Show {#MyAppName} help and usage information"; Flags: nowait postinstall skipifsilent shellexec
Filename: "{app}\{#MyAppExeName}"; Parameters: "--list-interfaces"; Description: "List available network interfaces"; Flags: nowait postinstall skipifsilent unchecked shellexec
Filename: "{app}\{#MyAppExeName}"; Parameters: "--dashboard"; Description: "Launch interactive dashboard (requires admin privileges)"; Flags: nowait postinstall skipifsilent unchecked shellexec; Check: IsAdminLoggedOn

; Open documentation
Filename: "{app}\README.md"; Description: "Open User Guide"; Flags: nowait postinstall skipifsilent unchecked shellexec

[UninstallDelete]
; Clean up user data and configuration files
Type: files; Name: "{userappdata}\packet_sniffer\*"
Type: dirifempty; Name: "{userappdata}\packet_sniffer"
Type: files; Name: "{localappdata}\packet_sniffer\logs\*"
Type: dirifempty; Name: "{localappdata}\packet_sniffer\logs"
Type: files; Name: "{localappdata}\packet_sniffer\exports\*"
Type: dirifempty; Name: "{localappdata}\packet_sniffer\exports"
Type: dirifempty; Name: "{localappdata}\packet_sniffer"
Type: files; Name: "{app}\logs\*"
Type: dirifempty; Name: "{app}\logs"
Type: files; Name: "{app}\exports\*" 
Type: dirifempty; Name: "{app}\exports"

[Registry]
; Add to PATH if requested
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Tasks: addtopath; Check: NeedsAddPath('{app}')

; Windows Firewall exceptions
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\StandardProfile\AuthorizedApplications\List"; ValueType: string; ValueName: "{app}\{#MyAppExeName}"; ValueData: "{app}\{#MyAppExeName}:*:Enabled:{#MyAppName}"; Tasks: firewall
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\DomainProfile\AuthorizedApplications\List"; ValueType: string; ValueName: "{app}\{#MyAppExeName}"; ValueData: "{app}\{#MyAppExeName}:*:Enabled:{#MyAppName}"; Tasks: firewall

; Uninstall registry cleanup
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata}"; Tasks: addtopath; Flags: uninsdeletevalue; Check: UninstallNeedsRemovePath('{app}')
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\StandardProfile\AuthorizedApplications\List"; ValueName: "{app}\{#MyAppExeName}"; Tasks: firewall; Flags: uninsdeletevalue
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\DomainProfile\AuthorizedApplications\List"; ValueName: "{app}\{#MyAppExeName}"; Tasks: firewall; Flags: uninsdeletevalue

[Code]
// Global variables
var
  DependencyPage: TOutputMsgMemoWizardPage;
  
// Function to check if running as administrator
function InitializeSetup(): Boolean;
begin
  Result := True;
  // Check if running as administrator
  if not IsAdminLoggedOn then
  begin
    MsgBox('This application requires administrator privileges to capture network packets.' + #13#10 + 
           'Please run the installer as an administrator.' + #13#10#13#10 +
           'Right-click the installer and select "Run as administrator".', mbError, MB_OK);
    Result := False;
  end;
end;

// Function to create custom wizard pages
procedure InitializeWizard();
begin
  // Create dependency information page
  DependencyPage := CreateOutputMsgMemoPage(wpSelectTasks,
    'System Dependencies', 'Required components for network packet capture',
    'The following components are required for packet capture functionality:' + #13#10#13#10 +
    '• Npcap Driver: Required for packet capture on Windows' + #13#10 +
    '• Microsoft Visual C++ Redistributable 2015-2022 (x64)' + #13#10 +
    '• Windows Administrator Privileges' + #13#10#13#10 +
    'If Npcap is not installed, please download it from:' + #13#10 +
    'https://npcap.com/ and install it before using the packet sniffer.' + #13#10#13#10 +
    'The installer will now check for these dependencies...',
    '');
end;

// Function to check if path needs to be added
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  // Look for the path with leading and trailing semicolon and with or without backslash
  // Pos() returns 0 if not found
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
  if Result = True then
    Result := Pos(';' + UpperCase(Param) + '\;', ';' + UpperCase(OrigPath) + ';') = 0;
end;

// Function to check if path needs to be removed during uninstall
function UninstallNeedsRemovePath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := False;
    exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') > 0;
  if Result = False then
    Result := Pos(';' + UpperCase(Param) + '\;', ';' + UpperCase(OrigPath) + ';') > 0;
end;

// Function to check if Visual C++ Redistributable needs to be installed
function VCRedistNeedsInstall: Boolean;
begin
  // Check if Visual C++ 2015-2022 Redistributable x64 is installed
  Result := not RegKeyExists(HKEY_LOCAL_MACHINE,
    'SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64');
  if Result then
    Result := not RegKeyExists(HKEY_LOCAL_MACHINE,
      'SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64');
end;

// Function to check if Npcap is installed
function IsNpcapInstalled: Boolean;
begin
  Result := RegKeyExists(HKEY_LOCAL_MACHINE, 'SOFTWARE\Npcap') or
            RegKeyExists(HKEY_LOCAL_MACHINE, 'SOFTWARE\WOW6432Node\Npcap') or
            FileExists(ExpandConstant('{sys}\Npcap\wpcap.dll')) or
            FileExists(ExpandConstant('{sys}\wpcap.dll'));
end;

// Post-installation setup
procedure CurStepChanged(CurStep: TSetupStep);
var
  ResultCode: Integer;
begin
  if CurStep = ssPostInstall then
  begin
    // Create application data directories
    CreateDir(ExpandConstant('{userappdata}\packet_sniffer'));
    CreateDir(ExpandConstant('{userappdata}\packet_sniffer\exports'));
    CreateDir(ExpandConstant('{userappdata}\packet_sniffer\logs'));
    CreateDir(ExpandConstant('{localappdata}\packet_sniffer'));
    CreateDir(ExpandConstant('{localappdata}\packet_sniffer\exports'));
    CreateDir(ExpandConstant('{localappdata}\packet_sniffer\logs'));
    CreateDir(ExpandConstant('{app}\logs'));
    CreateDir(ExpandConstant('{app}\exports'));
    
    // Check for Npcap installation
    if not IsNpcapInstalled then
    begin
      if MsgBox('Npcap driver is not detected on your system.' + #13#10#13#10 +
                'Npcap is required for packet capture functionality.' + #13#10 +
                'Would you like to download and install Npcap now?' + #13#10#13#10 +
                'You can also download it later from: https://npcap.com/',
                mbConfirmation, MB_YESNO) = IDYES then
      begin
        ShellExec('open', 'https://npcap.com/', '', '', SW_SHOWNORMAL, ewNoWait, ResultCode);
      end;
    end;
    
    // Install Visual C++ Redistributable if needed
    if VCRedistNeedsInstall and FileExists(ExpandConstant('{tmp}\vcredist_x64.exe')) then
    begin
      if MsgBox('Microsoft Visual C++ Redistributable is required.' + #13#10 +
                'Would you like to install it now?', mbConfirmation, MB_YESNO) = IDYES then
      begin
        Exec(ExpandConstant('{tmp}\vcredist_x64.exe'), '/quiet /norestart', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
      end;
    end;
  end;
end;

// Final installation check
procedure DeinitializeSetup();
begin
  if not IsNpcapInstalled then
  begin
    MsgBox('Installation completed successfully!' + #13#10#13#10 +
           'IMPORTANT: Npcap driver was not detected.' + #13#10 +
           'Please install Npcap from https://npcap.com/ before using the packet sniffer.' + #13#10#13#10 +
           'The application will not function without the Npcap driver.',
           mbInformation, MB_OK);
  end else
  begin
    MsgBox('Installation completed successfully!' + #13#10#13#10 +
           'You can now launch the Advanced Network Packet Sniffer from:' + #13#10 +
           '• Start Menu → ' + ExpandConstant('{#MyAppName}') + #13#10 +
           '• Desktop shortcut (if created)' + #13#10 +
           '• Command line with administrator privileges' + #13#10#13#10 +
           'For best results, always run as administrator.',
           mbInformation, MB_OK);
  end;
end;

[Messages]
WelcomeLabel1=Welcome to the [name] Setup Wizard
WelcomeLabel2=This will install [name/ver] on your computer.%n%nThis application provides enterprise-grade network packet analysis with advanced features:%n%n• Real-time packet capture and analysis%n• Advanced dashboard with threat detection%n• Interactive GUI with bandwidth visualization%n• Export capabilities (JSON/CSV)%n• Human-readable packet descriptions%n• Security monitoring and alerts%n• Connection flow tracking%n• Geographical IP analysis%n%nWARNING: This application requires administrator privileges and should only be used on networks you own or have permission to monitor.%n%nClick Next to continue, or Cancel to exit Setup.

[Components]
Name: "main"; Description: "Core Application"; Types: full compact custom; Flags: fixed
Name: "docs"; Description: "Documentation and Examples"; Types: full custom
Name: "config"; Description: "Configuration Templates"; Types: full custom
Name: "tools"; Description: "Additional Tools and Scripts"; Types: full custom

[Types]
Name: "full"; Description: "Full Installation (Recommended)"
Name: "compact"; Description: "Compact Installation"
Name: "custom"; Description: "Custom Installation"; Flags: iscustom

[InstallDelete]
Type: files; Name: "{app}\config.json"