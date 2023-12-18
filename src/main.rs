
//https://raw.githubusercontent.com/Triophore/portscan/main/ports.json
extern crate clap;
use std::{time::Duration, net::{TcpStream, ToSocketAddrs}, collections::HashMap};

use terminal_menu::{menu, label, button, run, mut_menu};
use clap::{arg, Command};






#[tokio::main]
async fn main() {
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
            full_scan(matches.get_one::<String>("h").unwrap().to_string()).await;
        }

        if matches.get_one::<String>("mode").unwrap() == "Q" {
            
        }

        


}


async fn full_scan(IPAddress : String){
    let target = IPAddress.as_str(); // Change this to the IP address you want to scan
    let timeout = Duration::from_millis(300); // Adjust the timeout duration as needed

    println!("Starting to download ports DB !");

    let case_study_url = format!("https://raw.githubusercontent.com/Triophore/portscan/main/ports.json");

    println!("Encoded URL {}",case_study_url.clone());

    match reqwest::get(case_study_url).await {
         Ok(resp) => {
            let json: serde_json::Value = resp.json().await.unwrap();
            //println!("{:?}",json);


            println!("Scanning ports on {}", target);

            let mut Res : HashMap<String,String> = HashMap::new();
        
            let port_range = 1..=65535;

            for port in port_range {
                let target_address = format!("{}:{}", target, port);
                match target_address.to_socket_addrs() {
                    Ok(mut addrs) => {
                        if let Some(target_addr) = addrs.next() {
                            //println!("Trying on {}",target_addr.clone());
                            if let Ok(socket) = TcpStream::connect_timeout(&target_addr,timeout) {
                                println!("Port {} is open", port);
                                // Additional actions for an open port can be added here
                                let inf =json.get(port.to_string());
                                if inf.is_some() {
                                    println!("{:?}",inf.unwrap().as_array()[0]);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to resolve the target address for port {}: {}", port, e);
                    }
                }
            }
         }
         Err(err) => {
            println!("Reqwest Error: {}", err)
         }
    }

 

     // Resolve the hostname to IP addresses
     
  

}
