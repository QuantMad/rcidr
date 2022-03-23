use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use clap::Parser;
use crate::cidr::Cidr;
use crate::cli::Cli;
use std::str::FromStr;
use std::io::BufRead;

mod cli;
mod cidr;

/// Command: run --package rcidr --bin rcidr -- -n 192.168.1.0/29

fn main() {
    let cli = Cli::parse();

    if let Some(open) = cli.open {
        let lines = read_lines(open);
        if let Some(export) = cli.export {
            let mut target = File::create(export)
                .expect("Ошибка создания файла");

            for line in lines.expect("Ошибка чтения") {
                let cidr = Cidr::from_str(&line.unwrap());
                //println!("{}", cidr.unwrap())
                for ip in cidr.unwrap() {
                    println!("{}", ip);
                    target.write(format!("{}\n", ip).as_ref());
                }
            }
        }
    }

    /*if let Some(network) = cli.network {
        //println!("{}", network.base);

        let mut file = File::create("/home/quantmad/list.txt")
            .expect("Ошибка создания файла");
        for ip in network.clone() {
            //println!("{ip}");
            let _ = file.write(format!("{}\n", ip).as_ref());
        }
    }*/
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
/*
fn export_list(path:String, network:Vec<Ipv4Addr>) {
    let mut file = File::create(path)
        .expect("Ошибка создания файла");
    for addr in network.to_vec() {
        let _ = file.write(format!("{}\n", addr).as_ref());
    }
}*/

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}