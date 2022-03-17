use std::fs::File;
use std::io::Write;
use std::net::Ipv4Addr;
use std::path::Path;
use clap::Parser;
use crate::cli::Cli;

mod cli;
mod cidr;

/// Command: run --package rcidr --bin rcidr -- -n 192.168.1.0/29

fn main() {
    let cli = Cli::parse();

    if let Some(network) = cli.network {
        println!("{}", network.base);

        for ip in network {
            println!("{ip}");
        }
    }
/*
    match cli.export {
        Some(path) => match cli.network {
            Some(network) => export_list(path,network.to_vec()),
            None => match cli.network {
                Some(ref network) => {
                    for addr in network.to_vec() {
                        println!("{addr}")
                    }
                },
                None => ()
            }
        },
        None => ()
    }*/
}

fn export_list(path:String, network:Vec<Ipv4Addr>) {
    let mut file = File::create(path)
        .expect("Ошибка создания файла");
    for addr in network.to_vec() {
        file.write(format!("{}\n", addr).as_ref());
    }
}