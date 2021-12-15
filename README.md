# DnsSeeker is compatible with the IPv4 and IPv6 protocol
A simple program called "DnsSeeker" which uses the dns-lookup library.
https://github.com/keeperofdakeys/dns-lookup/
<br/>
This simple program takes advantage of the possibilities offered by the Regex library.
https://github.com/rust-lang/regex
## Invitation :
I invite anyone who has further ideas
## Testing on : :test_tube:
`Linux, Windows 10/11, Windows Serwer 2022`
## In Linux :
```
git clone https://github.com/Curar/DnsSeeker.git
```
```
cargo b --release
```
## This simple program will sort the addresses that match the domain :
## Create software on Linux for Windows 10/11 and Windows Server 2022 to make the program compatible with the Windows family:
You mast have : https://www.mingw-w64.org/
In Debian or Ubuntu install :
```
apt install mingw-w64
```
In Arch Linux :
```
pacman -S mingw-w64
```
Next steeps :
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```
rustup install stable
```
```
rustup target add x86_64-pc-windows-gnu
```
```
rustup toolchain install stable-x86_64-pc-windows-gnu
```
```
cargo b --release --target x86_64-pc-windows-gnu
```
```
cd target/x86_64-pc-windows-gnu/release/
```
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
<br/>
Similar programs in Windows are: nslookup.exe but cannot sort ip addresses
<br/>
DnsSeeker is 100% compatible with Rust version: 1.56.1
