use crate::cidr::Cidr;

use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(name = "rclap",
author = "Автор: QuantMad",
version = "v1.0.0")]
pub struct Cli {
    #[clap(short, long, help="Указать сеть вручную (прим: 192.168.0.0/24")]
    pub network: Option<Cidr>,

    #[clap(short, long, help="Указать список сетей из файла")]
    pub open: Option<String>,

    #[clap(short, long, help="Путь для экспорта списка IP адресов")]
    pub export: Option<String>,

    #[clap(short, long, help="Дописывать ли экспортируемый файл")]
    pub append: bool,

    #[clap(short, long, help="Выводить ли список адресов в консоль")]
    pub print: bool,
}
