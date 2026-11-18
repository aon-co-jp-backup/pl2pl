; Java_to Windows installer (Inno Setup 6)
; ビルド: ISCC /DAppVersion=0.1.0 /DArch=x64 /DExeDir=..\target\release installer\java_to.iss
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
AppId={{4F8A2C61-7D3B-49E5-B0C2-5E91A6D3F7B8}
AppName=Java_to
AppVersion={#AppVersion}
AppPublisher=aon-co-jp
AppPublisherURL=https://github.com/aon-co-jp/Java_to
DefaultDirName={autopf}\Java_to
DefaultGroupName=Java_to
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\dist
OutputBaseFilename=Java_to-Setup-{#Arch}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\java_to.exe
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
Source: "{#ExeDir}\java_to.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Java_to"; Filename: "{app}\java_to.exe"; WorkingDir: "{userdocs}"
Name: "{group}\Java_to README"; Filename: "{app}\README.md"
Name: "{group}\{cm:UninstallProgram,Java_to}"; Filename: "{uninstallexe}"

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
