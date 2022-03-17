use clap::Parser;
use crate::cli::Cli;

mod cli;
mod cidr;

/// run --package rcidr --bin rcidr -- -n 192.168.1.0/29

fn main() {
    let cli = Cli::parse();

    match cli.network {
        Some(result) => {
            for addr in result.to_vec() {
                println!("{addr}")
            }
        },
        None => ()
    }
}
