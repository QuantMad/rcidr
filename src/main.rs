mod stat;

use std::net::{Ipv4Addr};
use std::str::FromStr;
use clap::{App, Arg};
use crate::stat::*;

fn main() {
    let network = IpNetworkArg::new();
    println!("{} / {}", network.addr, network.prefix);

    let a : u8 = 10; let b : u8 = 88; let c : u8 = 2; let d : u8 = 3;
    let n = ip4_to_u32(a, b, c, d);
    println!("{}.{}.{}.{} = {}", a, b, c, d, n);
    let i = u32_to_ip4(n).octets();
    println!("{} = {}.{}.{}.{}", n, i[0], i[1], i[2], i[3]);
    println!("{}",  cidr_addr_count(24));
}

struct IpNetworkArg {
    addr:Ipv4Addr,
    prefix:u8,
}

impl IpNetworkArg {
    fn new() -> Self {
        let app = App::new("rcidr");
        let arg = Arg::new("network")
            .long("network")
            .short('n')
            .takes_value(true)
            .required(true);
        let app = app.arg(arg);
        let matches = app.get_matches();

        let arg = matches.value_of("network")
            .expect("Укажи сеть в формате CIDR");

        if arg.contains("/") {
            let pare = arg.split('/').collect::<Vec<&str>>();
            let raw_addr = pare[0];
            let raw_prefix = pare[1];
            let addr = Ipv4Addr::from_str(raw_addr)
                .expect("Некорректный формат ввода IP адреса");
            let prefix = u8::from_str(raw_prefix)
                .expect("Некорректный формат ввода префикса");

            IpNetworkArg{
                addr,
                prefix
            }
        } else {
            panic!("Некорректный формат ввода")
        }
    }
}