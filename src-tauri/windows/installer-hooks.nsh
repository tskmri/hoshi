; Extra NSIS hooks for the Hoshi installer.
;
; Tauri's template always creates the Start Menu shortcut, but only creates a
; desktop shortcut for silent and passive installs. Creating it here means a
; normal interactive install also leaves a shortcut on the desktop, so Windows
; search can find and launch Hoshi without opening the Start Menu.

!macro NSIS_HOOK_POSTINSTALL
  CreateShortcut "$DESKTOP\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "" "$INSTDIR\${MAINBINARYNAME}.exe" 0 1 "" "Hoshi - Japanese EPUB reader"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$DESKTOP\${PRODUCTNAME}.lnk"
!macroend
