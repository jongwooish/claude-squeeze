; Inno Setup script for squeeze-Auditor (Windows).
; Built by .github/workflows/release.yml:
;   iscc /DAppVersion=0.2.0 /DArch=x64 /DBinDir=<folder with squeeze.exe and squeeze-auditor.exe> /DOutDir=dist squeeze-auditor.iss
; Installs for the current user only (no admin rights), into the same folder
; that v0.1's install.ps1 used, so an existing install is upgraded in place.

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif
#ifndef Arch
  #define Arch "x64"
#endif
#ifndef BinDir
  #define BinDir "..\..\..\..\CARGO\target\release"
#endif
#ifndef OutDir
  #define OutDir "dist"
#endif

[Setup]
AppId={{6C1E5B7A-2F3D-4B8E-9A61-5E0D7C2A9F14}
AppName=squeeze-Auditor
AppVersion={#AppVersion}
AppVerName=squeeze-Auditor {#AppVersion}
AppPublisher=claude-squeeze
AppPublisherURL=https://github.com/jongwooish/claude-squeeze
AppSupportURL=https://github.com/jongwooish/claude-squeeze/issues
DefaultDirName={localappdata}\Programs\squeeze
DisableDirPage=yes
DefaultGroupName=squeeze-Auditor
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ChangesEnvironment=yes
CloseApplications=yes
OutputDir={#OutDir}
OutputBaseFilename=squeeze-Auditor-{#AppVersion}-setup-{#Arch}
SetupIconFile=..\icon\icon.ico
UninstallDisplayIcon={app}\squeeze-auditor.ico
UninstallDisplayName=squeeze-Auditor
LicenseFile=..\..\LICENSE
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
#if Arch == "arm64"
ArchitecturesAllowed=arm64
ArchitecturesInstallIn64BitMode=arm64
#else
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
#endif

[Languages]
Name: "en"; MessagesFile: "compiler:Default.isl"
Name: "ko"; MessagesFile: "compiler:Languages\Korean.isl"
Name: "ja"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "fr"; MessagesFile: "compiler:Languages\French.isl"
Name: "es"; MessagesFile: "compiler:Languages\Spanish.isl"

[CustomMessages]
en.ConnectCC=Connect to Claude Code now (adds hooks for all projects)
ko.ConnectCC=지금 Claude Code에 연결 (모든 프로젝트에 훅 추가)
ja.ConnectCC=今すぐ Claude Code に接続する（すべてのプロジェクトにフックを追加）
fr.ConnectCC=Connecter à Claude Code maintenant (ajoute les hooks pour tous les projets)
es.ConnectCC=Conectar con Claude Code ahora (añade los hooks a todos los proyectos)
en.OpenApp=Open squeeze-Auditor
ko.OpenApp=squeeze-Auditor 열기
ja.OpenApp=squeeze-Auditor を開く
fr.OpenApp=Ouvrir squeeze-Auditor
es.OpenApp=Abrir squeeze-Auditor
en.DesktopIcon=Create a desktop shortcut
ko.DesktopIcon=바탕 화면에 바로 가기 만들기
ja.DesktopIcon=デスクトップにショートカットを作成
fr.DesktopIcon=Créer un raccourci sur le bureau
es.DesktopIcon=Crear un acceso directo en el escritorio

[Tasks]
Name: "desktopicon"; Description: "{cm:DesktopIcon}"; Flags: unchecked

[Files]
Source: "{#BinDir}\squeeze.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#BinDir}\squeeze-auditor.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\icon\icon.ico"; DestDir: "{app}"; DestName: "squeeze-auditor.ico"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion

[Icons]
Name: "{userprograms}\squeeze-Auditor"; Filename: "{app}\squeeze-auditor.exe"; IconFilename: "{app}\squeeze-auditor.ico"; Comment: "See and control what squeeze saves in Claude Code and the Claude desktop app"
Name: "{userdesktop}\squeeze-Auditor"; Filename: "{app}\squeeze-auditor.exe"; IconFilename: "{app}\squeeze-auditor.ico"; Tasks: desktopicon

[Registry]
; add the install folder to the user PATH once, so `squeeze` works in terminals
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: NeedsAddPath(ExpandConstant('{app}'))

[Run]
Filename: "{app}\squeeze.exe"; Parameters: "setup --claude-code"; Description: "{cm:ConnectCC}"; Flags: postinstall runhidden
Filename: "{app}\squeeze-auditor.exe"; Description: "{cm:OpenApp}"; Flags: postinstall nowait skipifsilent

[UninstallRun]
; take squeeze out of Claude Code and the desktop app (originals are backed up in ~/.squeeze/backups)
Filename: "{app}\squeeze.exe"; Parameters: "setup --remove"; Flags: runhidden; RunOnceId: "RemoveHooks"

[Code]
function NeedsAddPath(Dir: string): Boolean;
var
  Path: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Uppercase(Dir) + ';', ';' + Uppercase(Path) + ';') = 0;
end;

procedure RemovePath(Dir: string);
var
  Path, Upper: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
    exit;
  Path := ';' + Path + ';';
  Upper := Uppercase(Path);
  P := Pos(';' + Uppercase(Dir) + ';', Upper);
  if P = 0 then
    exit;
  Delete(Path, P, Length(Dir) + 1);
  Path := Copy(Path, 2, Length(Path) - 2);
  RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    RemovePath(ExpandConstant('{app}'));
end;
