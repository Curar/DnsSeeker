# DnsSeeker
A simple program called "DnsSeeker" which uses the dns-lookup library.
https://github.com/keeperofdakeys/dns-lookup/
<br/>
This simple program takes advantage of the possibilities offered by the Regex library.
https://github.com/rust-lang/regex
## Testing on : :test_tube:
`Linux, Windows 10 , Windows Serwer 2022`
:stethoscope:
## In Linux :
```
cargo b --release
```
![GitHub Logo](dnsseeker-linux.png)
## Create software on Linux and Windows 10 and Windows Server 2022 to make the program compatible with the Windows family:
```
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
cargo b --release --target x86_64-pc-windows-gnu
```
![GitHub Logo](dnsseeker.png)
<br/>
I do not have a certificate, so my application is unsigned, therefore Windows SmartScreen detects it as dangerous.
If you want to be SmartScreen compatible use SignTool.exe (Signature Tool) :
<br/>
https://docs.microsoft.com/pl-pl/dotnet/framework/tools/signtool-exe
<br/>
https://docs.microsoft.com/pl-pl/windows/win32/seccrypto/using-signtool-to-sign-a-file
<br/>
To reduce the size of an executable use UPX ::arrow_double_down:
<br/>
https://github.com/upx/upx/releases
<br/>
Read more about optimization in Rust :
<br/>
https://doc.rust-lang.org/book/ch14-01-release-profiles.html
