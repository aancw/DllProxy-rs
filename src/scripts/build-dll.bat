:: adaptation from https://stackoverflow.com/questions/46732926/how-can-i-find-the-latest-visual-studio-developer-command-prompt-in-a-batch-file

@echo off
:: rem vsvarsall.bat does not work if there are quoted paths on %PATH%
set path=%path:"=%
set ARCH=%1
set DLL_LOC=%2
set OUT_FILE=%3
:: rem this will work for non VS 2017 build machines
if exist "c:\progra~2\Micros~1.0\vc\vcvarsall.bat" (
    call c:\progra~2\Micros~1.0\vc\vcvarsall.bat && goto :SetVSEnvFinished
)

:: echo vcvarsall.bat not found, looking for vsdevcmd.bat
:: rem Find and run vsdevcmd.bat
set "VS_PATH=C:\Program Files (x86)\Microsoft Visual Studio\2017"

:: rem The 2017 folder will not be present in Visual Studio 2017 Preview machines (such as 15.8 preview)
if not exist "%VS_PATH%" (
    set "VS_PATH=C:\Program Files (x86)\Microsoft Visual Studio"
)

if not exist "%VS_PATH%" (
    echo "%VS_PATH%" not found. Is Visual Studio installed? && goto :ErrorExit
)

for /f "delims=" %%F in ('dir /b /s "%VS_PATH%\vcvarsall.bat" 2^>nul') do set VSDEVCMD_PATH=%%F
echo ********Setup build environment********

setlocal
call "%VSDEVCMD_PATH%" %ARCH%
echo [*] Compiling DLL file
cl /nologo /LD %DLL_LOC% /link /out:%OUT_FILE%
endlocal

goto :SuccessExit


:ErrorExit
exit /b 1

:SuccessExit
exit /b 0

:SetVSEnvFinished