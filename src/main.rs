mod cidr;

use std::fs::File;
use std::io::Write;
use std::net::Ipv4Addr;
use clap::{Parser};
use crate::cidr::Cidr;

fn main() {
    let res = Cli::parse();
    match res.export_to {
        Some(export_to) => write_to(&export_to, res.cidr.to_vec()),
        None => ()
    }
}

fn write_to(path:&str, list:Vec<Ipv4Addr>) {
    let mut file = File::create(path).expect("Упс");
    for addr in list {
        let _ = file.write(format!("{}\n", addr.to_string()).as_bytes());
    }
}

#[derive(Parser, Debug)]
#[clap(name = "rcidr", author = "QuantMad", version = "0.1.0", about, long_about = None)]
pub struct Cli {
    #[clap(short = 'n', long = "network", name = "network")]
    cidr: Cidr,
    #[clap(short = 'e', long = "export", name = "export_to")]
    export_to:Option<String>,
}
