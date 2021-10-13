use std::env;
use std::io::{stdout, Write};
use std::net::IpAddr;
use std::net::TcpStream;
use std::process;

const MAX: u16 = 65535;

mod args;
use args::Args;

fn main() {
    let args: Vec<String> = env::args().collect();

    match Args::from_args(&args) {
        Ok(argument) => {
            scan(argument.ip_addr);
        }
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(0);
        }
    };
}

fn scan(addr: IpAddr) {
    let mut ports: Vec<u16> = Vec::new();

    for current_port in 0..MAX {
        match TcpStream::connect((addr, current_port)) {
            Ok(_) => {
                print!(".");
                stdout().flush().unwrap();
                ports.push(current_port);
            }
            Err(_) => {}
        };
    }

    println!();
    for port in &ports {
        println!("{} is open", port);
    }

    println!("{} ports open", ports.len())
}
