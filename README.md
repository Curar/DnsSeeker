# DnsSeeker
A simple program called "DnsSeeker" which uses the dns-lookup library
https://github.com/keeperofdakeys/dns-lookup/
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
cargo b --release --target x86_64-pc-windows-gnu
```
<br/>
If you want to be SmartScreen compatible use SignTool.exe (Signature Tool) :
<br/>
https://docs.microsoft.com/pl-pl/dotnet/framework/tools/signtool-exe
<br/>
To reduce the size of an executable use UPX ::arrow_double_down:
<br/>
https://github.com/upx/upx/releases
<br/>
I optimized the DnsSeeker code to :
<br/>
```
opt-level = 3
lto = true
```
<br/>
Read more about optimization in Rust :
<br/>
https://doc.rust-lang.org/book/ch14-01-release-profiles.html
