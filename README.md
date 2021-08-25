# DnsSeeker
DNS lookups in Rust
## Testing on : :test_tube:
`Linux, Windows 10 , Windows Serwer 2022, It probably works on macOS`
:stethoscope:
## In Linux :
```
cargo b --release
```
## Create software on Linux and Windows 10 and Windows Server 2022 to make the program compatible with the Windows family:
```
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
cargo build --target --release x86_64-pc-windows-gnu
```
## To reduce the size of an executable use UPX :
https://github.com/upx/upx/releases
I optimized the DnsSeeker code to opt-level = 3
