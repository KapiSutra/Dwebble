@echo off
cd /d "%~dp0"
REM 现在工作目录是.bat文件所在目录
cargo build --target x86_64-pc-windows-msvc --release