@echo off
title Launch MicroOS Interactive Console
echo ====================================================
echo   MICROOS LINUX - INTERACTIVE CONSOLE LAUNCHER
echo ====================================================
echo.

"C:\Program Files\Oracle\VirtualBox\VBoxManage.exe" list runningvms | findstr /i "MicroOS" >nul
if %errorlevel% equ 0 (
    echo [INFO] MicroOS is already running in background.
    echo [INFO] Restarting into a full visible Normal GUI window...
    "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe" controlvm "MicroOS" poweroff >nul 2>&1
    timeout /t 2 /nobreak >nul
)

echo [OK] Launching MicroOS in full interactive window...
start "" "C:\Program Files\Oracle\VirtualBox\VirtualBoxVM.exe" --startvm "MicroOS"
echo.
echo [DONE] The MicroOS window should now be open on your screen!
echo Check your Windows taskbar for the VirtualBox VM window.
timeout /t 4 >nul
