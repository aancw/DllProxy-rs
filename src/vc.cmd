@echo off
setlocal enabledelayedexpansion

for /f "usebackq tokens=*" %%i in (`vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
  set InstallDir=%%i
)

if exist "%InstallDir%\VC\Auxiliary\Build\Microsoft.VCToolsVersion.default.txt" (
  set /p Version=<"%InstallDir%\VC\Auxiliary\Build\Microsoft.VCToolsVersion.default.txt"

  rem Trim
  set Version=!Version: =!
)

if not "%Version%"=="" (
  rem Example hardcodes x64 as the host and target architecture, but you could parse it from arguments
  "%InstallDir%\VC\Tools\MSVC\%Version%\bin\HostX64\x64\cl.exe" %*
)