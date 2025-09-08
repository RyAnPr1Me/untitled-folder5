; Inno Setup Script for Advanced Network Packet Sniffer
; This script creates a Windows installer for the packet sniffer application

#define MyAppName "Advanced Network Packet Sniffer"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Packet Sniffer Team"
#define MyAppURL "https://github.com/RyAnPr1Me/packet-sniffer"
#define MyAppExeName "packet_sniffer.exe"
#define MyAppDescription "Advanced Network Packet Sniffer with user-friendly interface and real-time dashboard"

[Setup]
; NOTE: The value of AppId uniquely identifies this application.
AppId={{C7F2E5A1-8B4D-4A3F-9E2C-5D6F7A8B9C0D}
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
OutputDir=dist
OutputBaseFilename=PacketSnifferSetup-{#MyAppVersion}
SetupIconFile=icon.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1

[Files]
Source: "target\release\packet_sniffer.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "examples.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "config\default_config.json"; DestDir: "{app}\config"; Flags: ignoreversion
Source: "docs\*"; DestDir: "{app}\docs"; Flags: ignoreversion recursesubdirs createallsubdirs
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; Comment: "{#MyAppDescription}"
Name: "{group}\{#MyAppName} Dashboard"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--dashboard"; Comment: "Launch interactive dashboard mode"; IconFilename: "{app}\icon.ico"
Name: "{group}\List Network Interfaces"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--list-interfaces"; Comment: "Show available network interfaces"
Name: "{group}\User Guide"; Filename: "{app}\README.md"; Comment: "Read the user guide"
Name: "{group}\Examples"; Filename: "{app}\examples.md"; Comment: "View usage examples"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; Comment: "{#MyAppDescription}"
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: quicklaunchicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Parameters: "--generate-config"; WorkingDir: "{app}"; Flags: runhidden; Description: "Generate default configuration"
Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: files; Name: "{userappdata}\packet_sniffer\*"
Type: dirifempty; Name: "{userappdata}\packet_sniffer"

[Code]
function InitializeSetup(): Boolean;
begin
  Result := True;
  // Check if running as administrator
  if not IsAdminLoggedOn then
  begin
    MsgBox('This application requires administrator privileges to capture network packets. Please run the installer as an administrator.', mbError, MB_OK);
    Result := False;
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    // Create application data directories
    CreateDir(ExpandConstant('{userappdata}\packet_sniffer'));
    CreateDir(ExpandConstant('{userappdata}\packet_sniffer\exports'));
    CreateDir(ExpandConstant('{userappdata}\packet_sniffer\logs'));
  end;
end;

[Messages]
WelcomeLabel1=Welcome to the [name] Setup Wizard
WelcomeLabel2=This will install [name/ver] on your computer.%n%nThis application allows you to capture and analyze network packets with a user-friendly interface. It includes:%n%n• Real-time packet capture%n• Interactive dashboard%n• Export capabilities%n• Human-readable packet descriptions%n%nWARNING: This application requires administrator privileges and should only be used on networks you own or have permission to monitor.

[InstallDelete]
Type: files; Name: "{app}\config.json"