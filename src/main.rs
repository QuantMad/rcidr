mod cidr;

use clap::{Parser};
use crate::cidr::Cidr;

fn main() {
    let res = Cli::parse();
    println!("{}", res.cidr);
    for a in res.cidr.to_vec() {
        println!("{}", a);
    }

    match res.export_to {
        Some(export_to) => println!("{}", export_to),
        None => println!("нихуя")
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
