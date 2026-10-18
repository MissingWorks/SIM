@echo off
cd /d "%~dp0"
set SPIKE_MODE=prewarm
start "" "%~dp0target\release\spike2-tray.exe"
