use crate::stat::*;

use std::net::Ipv4Addr;
use std::str::FromStr;
use clap::{App, Arg};

pub struct IpNetworkArg {
    pub addr:Ipv4Addr,
    pub prefix:u8,
}

impl IpNetworkArg {
    pub fn new() -> Self {
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
            if prefix > 32 {
                panic!("Некорректный формат ввода префикса");
            }

            return IpNetworkArg{
                addr,
                prefix
            };
        } else {
            panic!("Некорректный формат ввода")
        }
    }

    pub fn to_vec(&self) -> Vec<Ipv4Addr> {
        let mut result = Vec::<Ipv4Addr>::new();
        for i in 0..cidr_addr_count(self.prefix) {
            let dig = ip4_to_u32(self.addr) + i;
            result.push(Ipv4Addr::from(u32_to_ip4(dig)));
        }

        return result;
    }
}