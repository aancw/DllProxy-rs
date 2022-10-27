# DllProxy-rs
Rust Implementation of SharpDllProxy for DLL Proxying Technique

## Features
- Everything in SharpDllProxy
- Automatic compile and build DLL

## Requirements
- Rust
- Cargo
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- Visual Studio 2019/2022 with Visual C++ Build Tools or Standalone Visual Studio Build Tools 2019/2022 from https://aka.ms/vs/17/release/vs_BuildTools.exe

## Build Instructions

1. Install Rust and cargo
2. Install cargo-make 
```
cargo install --force cargo-make
```
3. Build release app
```
cargo make -p release build-release
```
4. Run the app as described at Usage section.


## Usage
Before you can use it, you need to build first. Please read build instructions

- Help Information
```
PS C:>.\dll_proxy_rs.exe -h
DllProxy-rs 1.0
Petruknisme <me@petruknisme.com>
Rust Implementation of SharpDllProxy for DLL Proxying Technique

USAGE:
    dll_proxy_rs.exe [OPTIONS] --dll <DLL> --payload <PAYLOAD>

OPTIONS:
    -a, --auto                 Automatic DLL compilation
    -d, --dll <DLL>            Dll File Location to hijack
    -h, --help                 Print help information
    -p, --payload <PAYLOAD>    Shellcode file to insert in the hijacked dll
    -V, --version              Print version information
```

- Run without automatic dll compilation
```
.\dll_proxy_rs.exe -d <path/to/file.dll> -p <path/to/shellcode.bin>
```
- Run with automatic dll compilation
```
.\dll_proxy_rs.exe -d <path/to/file.dll> -p <path/to/shellcode.bin> -a
```
Note: For this example, I'm just using msfvenom to generate the payload

```
msfvenom -a x64 --platform windows -p windows/x64/messagebox TEXT="DLL Proxy Loading using Rust worked!" -f raw > shellcode.bin
```

## Demo

![](https://i.imgur.com/wqogZyE.gif)


## Thanks to
- Flangvik for his [SharpProxyDll](https://github.com/Flangvik/SharpDllProxy)

## References
- https://redteaming.co.uk/2020/07/12/dll-proxy-loading-your-favorite-c-implant/
- https://www.ired.team/offensive-security/persistence/dll-proxying-for-persistence

## License

MIT License

