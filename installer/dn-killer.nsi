Unicode true

!ifndef APP_VERSION
  !error "Pass the Cargo package version with /DAPP_VERSION"
!endif

!define APP_REGKEY "Software\DN Killer\GPUI"
!define UNINSTALL_REGKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\DNKillerGPUI"

Name "DN Killer ${APP_VERSION}"
OutFile "..\target\DN-Killer-${APP_VERSION}-x64-setup.exe"
InstallDir "$LOCALAPPDATA\Programs\DN Killer"
InstallDirRegKey HKCU "${APP_REGKEY}" "InstallPath"
RequestExecutionLevel user
Icon "..\assets\icon.ico"
UninstallIcon "..\assets\icon.ico"
SetCompressor /SOLID lzma

Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles

Section "DN Killer"
  SetShellVarContext current
  SetOutPath "$INSTDIR"
  File "..\target\x86_64-pc-windows-msvc\release\dn-killer.exe"
  WriteUninstaller "$INSTDIR\uninstall.exe"

  CreateDirectory "$SMPROGRAMS\DN Killer"
  CreateShortCut "$SMPROGRAMS\DN Killer\DN Killer.lnk" "$INSTDIR\dn-killer.exe"
  CreateShortCut "$SMPROGRAMS\DN Killer\Uninstall.lnk" "$INSTDIR\uninstall.exe"

  WriteRegStr HKCU "${APP_REGKEY}" "InstallPath" "$INSTDIR"
  WriteRegStr HKCU "${UNINSTALL_REGKEY}" "DisplayName" "DN Killer"
  WriteRegStr HKCU "${UNINSTALL_REGKEY}" "DisplayVersion" "${APP_VERSION}"
  WriteRegStr HKCU "${UNINSTALL_REGKEY}" "DisplayIcon" "$INSTDIR\dn-killer.exe,0"
  WriteRegStr HKCU "${UNINSTALL_REGKEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINSTALL_REGKEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKCU "${UNINSTALL_REGKEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINSTALL_REGKEY}" "NoRepair" 1
SectionEnd

Section "Uninstall"
  SetShellVarContext current
  Delete "$SMPROGRAMS\DN Killer\DN Killer.lnk"
  Delete "$SMPROGRAMS\DN Killer\Uninstall.lnk"
  RMDir "$SMPROGRAMS\DN Killer"

  Delete "$INSTDIR\dn-killer.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "${UNINSTALL_REGKEY}"
  DeleteRegKey HKCU "${APP_REGKEY}"
SectionEnd
