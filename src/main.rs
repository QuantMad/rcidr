mod cidr;
mod stat;

use clap::{Parser};
use crate::cidr::Cidr;

fn main() {
    let res = Cli::parse();
    println!("{}", res.cidr);
    for a in res.cidr.to_vec() {
        println!("{}", a);
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short = 'n', long = "network", name = "network")]
    cidr: Cidr,
}

