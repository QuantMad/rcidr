use crate::cidr::Cidr;

use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(name = "rclap",
author = "QuantMad",
version = "0.1.0")]
pub struct Cli {
    #[clap(short, long)]
    pub network: Option<Cidr>,
}