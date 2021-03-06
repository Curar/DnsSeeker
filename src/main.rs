//  DnsSeeker ⍟ (Brotherhood of Steel ) 
//  DNS lookups
//  writing for fun ✈
//  by Curar 2021 ☠
//  Writing on Linux in the Vim editor

use dns_lookup::lookup_host;
use regex::Regex;
use std::io;
use std::io::Write;

fn main() {
  
    let rootdns ="
        a.root-servers.net 198.41.0.4, 2001:503:ba3e::2:30   Verisign, Inc.
        b.root-servers.net 199.9.14.201, 2001:500:200::b     University of Southern California
        c.root-servers.net 192.33.4.12, 2001:500:2::c        Cogent Communications
        d.root-servers.net 199.7.91.13, 2001:500:2d::d       University of Maryland
        e.root-servers.net 192.203.230.10, 2001:500:a8::e    NASA (Ames Research Center)
        f.root-servers.net 192.5.5.241, 2001:500:2f::f       Internet Systems Consortium, Inc.
        g.root-servers.net 192.112.36.4, 2001:500:12::d0d    US Department of Defense (NIC)
        h.root-servers.net 198.97.190.53, 2001:500:1::53     US Army (Research Lab)
        i.root-servers.net 192.36.148.17, 2001:7fe::53       Netnod
        j.root-servers.net 192.58.128.30, 2001:503:c27::2:30 Verisign, Inc.
        k.root-servers.net 193.0.14.129, 2001:7fd::1         RIPE NCC
        l.root-servers.net 199.7.83.42, 2001:500:9f::42      ICANN
        m.root-servers.net 202.12.27.33, 2001:dc3::35        WIDE Project
        ";
    
    let info_art ="
        +-----------------------------------------+
        |                                         |
        |       Welcome to the Curar project      |
        |         https://github.com/curar        |
        |                                         |
        +-----------------------------------------+
        ";


    let disp_info ="
        (r) to display Root DNS
        (i) to display info.
        (q) to exit
        ";

    println!("\nDnsSeeker\n");

    loop {
    let mut adres=String::new();
    print!("Please enter a valid domain (google.com) : ");
    match io::stdout().flush() {
        Ok(_) => print!(""),
        Err(_) => continue,
        }
    match io::stdin().read_line(&mut adres) {
        Ok(_) => {
            adres = adres.trim().to_string();
                if adres.len() != 0 {
                    if let Some('\n')=adres.chars().next_back() {
                        adres.pop();
                    }
                    if let Some('\r')=adres.chars().next_back() {
                        adres.pop();
                    }
                    let warunek: bool = Regex::new(r"([a-z0-9A-Z]\.)*[a-z0-9-]+\.([a-z0-9]{2,24})+(\.co\.([a-z0-9]{2,24})|\.([a-z0-9]{2,24}))*")
                        .unwrap().is_match(&adres);
                    if adres == "q" {
                        println!("Goodbye!");
                        break;
                    } else if adres == "r" {
                        println!("{}", rootdns);
                    } else if adres == "i" {
                        println!("{}", info_art);
                    } else if warunek == true {
                        println!("You typed domain : {}", adres);
                        let mut ips: Vec<std::net::IpAddr> = lookup_host(&adres).unwrap();
                        ips.sort();
                        println!("\nResult :\n\n{}", ips.iter().fold(String::new(), |acc, &nawiasy| acc + &nawiasy.to_string() + "\n"));
                        println!("{}", disp_info);
                    } else {
                        println!("Attention! Please enter a valid domain");
                        }
                    
                } else {
                    println!("Attention! Please enter a valid domain");
                    }
        }
        Err(_) => continue,
    }
  }
}
