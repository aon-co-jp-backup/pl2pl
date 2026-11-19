@echo off
rem pl2pl watch for Windows Task Scheduler (ASCII only). Nightly and morning, e.g.:
rem   schtasks /Create /SC DAILY /ST 03:00 /TN pl2pl-watch-night /TR "F:\pl2pl\scripts\pl2pl-watch.cmd"
rem   schtasks /Create /SC DAILY /ST 07:00 /TN pl2pl-watch-morning /TR "F:\pl2pl\scripts\pl2pl-watch.cmd"
rem Needs: bash (Git for Windows) on PATH, version-control built at F:\version-control, SMTP env vars for the notify mail.
set "PATH=%USERPROFILE%\.cargo\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;%PATH%"
cd /d "%~dp0.."
echo ===== %DATE% %TIME% >> watch.log
target\release\pl2pl.exe watch --vc F:\version-control\target\release\version-control.exe >> watch.log 2>&1
echo exit=%ERRORLEVEL% >> watch.log
