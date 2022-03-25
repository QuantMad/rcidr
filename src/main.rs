use std::fs::File;
use std::{fs, io};
use std::path::Path;
use clap::Parser;
use crate::cidr::Cidr;
use crate::cli::Cli;
use std::str::FromStr;
use std::io::{BufRead, Write};

mod cli;
mod cidr;

/// Command: run --package rcidr --bin rcidr -- -n 192.168.1.0/29

fn export(cli: &Cli, cidr_list: Vec<Cidr>) {
    let path = cli.export.clone().unwrap();
    let mut file = if fs::metadata(path.clone()).is_ok() &&
        cli.append.clone() {
        File::options()
            .read(true)
            .write(true)
            .append(true)
            .open(path)
            .expect("Ошибка чтения целевого файла")
    } else {
        File::create(path)
            .expect("Ошибка создания файла")
    };

    for cidr in cidr_list {
        for ip in cidr {
            let _ = file.write_all(format!("{}\n", ip).as_bytes());
            if cli.print.clone() {
                println!("{}", ip);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if (cli.network.is_some() && cli.open.is_some()) ||
        (!cli.network.is_some() && !cli.open.is_some()) {
        panic!("Конфликт флагов: -n и -o нельзя использовать одновременно")
    }

    let cidr_list = parse_cidr_list(&cli);

    if cli.export.is_some() {
        export(&cli, cidr_list);
    } else if cli.print.clone() {
        for cidr in cidr_list {
            for ip in cidr {
                println!("{}", ip);
            }
        }
    }
}

fn parse_cidr_list(cli: &Cli) -> Vec<Cidr> {
    let mut cidr_list = Vec::<Cidr>::new();

    if cli.network.is_some() {
        cidr_list.push(cli.network.clone().unwrap())
    } else if cli.open.is_some() {
        let lines = read_lines(cli.open.clone().unwrap());
        for line in lines.expect("Ошибка чтения") {
            let cidr = Cidr::from_str(&line.unwrap());
            cidr_list.push(cidr.unwrap());
        }
    }

    cidr_list
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
