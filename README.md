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
cargo b --release --target x86_64-pc-windows-gnu
```
## To reduce the size of an executable use UPX ::arrow_double_down:
https://github.com/upx/upx/releases
<br/>
I optimized the DnsSeeker code to :
<br/>
```
opt-level = 3
lto = true
```
## DNS Root Servers (Top 13):
```
a.root-servers.net 	198.41.0.4, 2001:503:ba3e::2:30     Verisign, Inc.
b.root-servers.net 	199.9.14.201, 2001:500:200::b       University of Southern California, Information Sciences Institute
c.root-servers.net 	192.33.4.12, 2001:500:2::c          Cogent Communications
d.root-servers.net 	199.7.91.13, 2001:500:2d::d         University of Maryland
e.root-servers.net 	192.203.230.10, 2001:500:a8::e      NASA (Ames Research Center)
f.root-servers.net 	192.5.5.241, 2001:500:2f::f         Internet Systems Consortium, Inc.
g.root-servers.net 	192.112.36.4, 2001:500:12::d0d      US Department of Defense (NIC)
h.root-servers.net 	198.97.190.53, 2001:500:1::53       US Army (Research Lab)
i.root-servers.net 	192.36.148.17, 2001:7fe::53         Netnod
j.root-servers.net 	192.58.128.30, 2001:503:c27::2:30   Verisign, Inc.
k.root-servers.net 	193.0.14.129, 2001:7fd::1           RIPE NCC
l.root-servers.net 	199.7.83.42, 2001:500:9f::42        ICANN
m.root-servers.net 	202.12.27.33, 2001:dc3::35          WIDE Project
```
