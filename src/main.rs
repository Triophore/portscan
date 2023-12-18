
//https://raw.githubusercontent.com/Triophore/portscan/main/ports.json
extern crate clap;
use std::{time::Duration, net::{TcpStream, ToSocketAddrs}, collections::HashMap};

use terminal_menu::{menu, label, button, run, mut_menu};
use clap::{arg, Command};







fn main() {
    println!("Portscanner");
    println!("Developed and managed by Triophore");
    

    let matches = Command::new("MyApp")
    .version("1.0")
    .author("Triophore ")
    .about("Port Scanner")
    .arg(arg!(--h <VALUE>).required(true))
    .arg(arg!(--mode <VALUE>).required(true))
    .get_matches();

        println!(
            "host: {:?}",
            matches.get_one::<String>("h").expect("required")
        );

        println!(
            "mode: {:?}",
            matches.get_one::<String>("mode").expect("required")
        );


        if matches.get_one::<String>("mode").unwrap() == "F" {
            full_scan(matches.get_one::<String>("h").unwrap().to_string())
        }

        if matches.get_one::<String>("mode").unwrap() == "Q" {
            
        }


    // let menu = menu(vec![

    //     // label:
    //     //  not selectable, useful as a title, separator, etc...
    //     label("----------------------"),
    //     label("terminal-menu"),
    //     label("use wasd or arrow keys"),
    //     label("enter to select"),
    //     label("'q' or esc to exit"),
    //     label("-----------------------"),

    //     // button:
    //     //  exit the menu
    //     button("Popular Ports (quick)"),
    //     button("Full Ports (slow)"),
    //     // button("Charlie")

    // ]);
    // run(&menu);

    // // you can get the selected buttons name like so:
    // println!("Selected: {}", mut_menu(&menu).selected_item_name());

    // let op = String::from(mut_menu(&menu).selected_item_name());

    // match op.as_ref() {
    //     "Full Ports (slow)" => {
    //         println!("full scan started !");
    //     },
    //     "Popular Ports (quick)" => {
    //         println!("quick scan started !");
    //     },
    //     _ =>{
    //         println!("Invalid Option");
    //         std::process::exit(0);
    //     }
    // }

}


fn full_scan(IPAddress : String){
    let target = IPAddress.as_str(); // Change this to the IP address you want to scan
    let timeout = Duration::from_millis(100); // Adjust the timeout duration as needed

    println!("Starting to download ports DB !{}");

    

    println!("Scanning ports on {}", target);

    let mut Res : HashMap<String,String> = HashMap::new();

    let port_range = 1..=65535; 

     // Resolve the hostname to IP addresses
     
     for port in port_range {
        let target_address = format!("{}:{}", target, port);
        match target_address.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(target_addr) = addrs.next() {
                    //println!("Trying on {}",target_addr.clone());
                    if let Ok(socket) = TcpStream::connect_timeout(&target_addr,timeout) {
                        println!("Port {} is open", port);
                        // Additional actions for an open port can be added here
                    }
                }
            }
            Err(e) => {
                println!("Failed to resolve the target address for port {}: {}", port, e);
            }
        }
    }

}
