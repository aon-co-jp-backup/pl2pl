; COBOL_to Windows installer (Inno Setup 6)
; ビルド: ISCC /DAppVersion=0.1.0 /DArch=x64 /DExeDir=..\target\release installer\cobol_to.iss
#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif
#ifndef Arch
  #define Arch "x64"
#endif
#ifndef ExeDir
  #define ExeDir "..\target\release"
#endif

[Setup]
AppId={{6B1E4D7A-2C53-4F0E-9A31-C0B07A2F5E10}
AppName=COBOL_to
AppVersion={#AppVersion}
AppPublisher=aon-co-jp
AppPublisherURL=https://github.com/aon-co-jp/COBOL_to
DefaultDirName={autopf}\COBOL_to
DefaultGroupName=COBOL_to
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\dist
OutputBaseFilename=COBOL_to-Setup-{#Arch}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\cobol_to.exe
LicenseFile=..\LICENSE
#if Arch == "arm64"
ArchitecturesAllowed=arm64
ArchitecturesInstallIn64BitMode=arm64
#else
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
#endif

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[Tasks]
Name: "addpath"; Description: "PATH に追加 / Add to PATH"; Flags: unchecked

[Files]
Source: "{#ExeDir}\cobol_to.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\NOTIFY.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\COBOL_to"; Filename: "{app}\cobol_to.exe"; WorkingDir: "{userdocs}"
Name: "{group}\COBOL_to README"; Filename: "{app}\README.md"
Name: "{group}\{cm:UninstallProgram,COBOL_to}"; Filename: "{uninstallexe}"

[Registry]
Root: HKA; Subkey: "{code:EnvKey}"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Tasks: addpath; Check: NeedsPath(); Flags: preservestringtype

[Code]
function EnvKey(Param: String): String;
begin
  if IsAdminInstallMode then
    Result := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment'
  else
    Result := 'Environment';
end;

function NeedsPath(): Boolean;
var P: String; Root: Integer;
begin
  if IsAdminInstallMode then Root := HKLM else Root := HKCU;
  if not RegQueryStringValue(Root, EnvKey(''), 'Path', P) then P := '';
  Result := Pos(';' + Lowercase(ExpandConstant('{app}')) + ';', ';' + Lowercase(P) + ';') = 0;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var P, A: String; Root, I: Integer;
begin
  if CurUninstallStep <> usPostUninstall then Exit;
  if IsAdminInstallMode then Root := HKLM else Root := HKCU;
  if not RegQueryStringValue(Root, EnvKey(''), 'Path', P) then Exit;
  A := ';' + ExpandConstant('{app}');
  I := Pos(Lowercase(A), Lowercase(P));
  if I > 0 then begin
    Delete(P, I, Length(A));
    RegWriteExpandStringValue(Root, EnvKey(''), 'Path', P);
  end;
end;
