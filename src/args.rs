use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Args {
    pub ip_addr: IpAddr,
}

impl Args {
    pub fn from_args(args: &[String]) -> Result<Args, &str> {
        match args.len() {
            len if len == 2 => {
                let argument = &args[1][..];

                if let Ok(ip_addr) = IpAddr::from_str(argument) {
                    Ok(Args { ip_addr })
                } else {
                    println!("Usage: port_sniff <ip>");
                    Err("Unknown Argument")
                }
            }
            _ => Err("Not enough arguments"),
        }
    }
}
