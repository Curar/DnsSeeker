//  DnsSeeker 
//  writing for fun ✈
//  by Curar 2021 ☠
//  Writing on Linux in the Vim editor

use dns_lookup::lookup_host;
use std::io;
use std::io::Write;
use std::thread;
use std::time;

fn main() {
   
    println!("DnsSeeker\n\n\n");

    loop {
    
    let mut adres=String::new();
    
    print!("Please enter a valid domain (google.com) : ");

    match io::stdout().flush() {
        Ok(_) => print!("") ,
        Err(e) => println!("{}", e),
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
    
                 println!("You typed domain : {}", adres);
    
                
                let ips: Vec<std::net::IpAddr> = lookup_host(&adres).unwrap();
                
                println!("\n{}", ips.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + "\n"));
                
                thread::sleep(time::Duration::from_millis(500));

                println!("\nPress (Ctrl + C) to exit . . .");


              } else {
                println!("Attention! Please enter a valid domain");
               }
        }
        Err(error) => println!("{}", error),
    }
  }
}
