use crate::stat::*;

use std::fmt::{Debug, Formatter};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::string::ParseError;
use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Cidr {
    base:Ipv4Addr,
    prefix:u8,
}

impl Cidr {
    pub fn to_vec(&self) -> Vec<Ipv4Addr> {
        let mut result = Vec::<Ipv4Addr>::new();
        for i in 0..cidr_addr_count(self.prefix) {
            let dig = ip4_to_u32(self.base) + i;
            result.push(Ipv4Addr::from(u32_to_ip4(dig)));
        }

        return result;
    }
}

impl FromStr for Cidr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return if s.contains("/") {
            let pare = s.split('/').collect::<Vec<&str>>();
            let base = Ipv4Addr::from_str(pare[0])
                .expect("Некорректный формат ввода IP адреса");
            let prefix = u8::from_str(pare[1])
                .expect("Некорректный формат ввода префикса");
            if prefix > 32 {
                panic!("Некорректный формат ввода префикса");
            }
            Ok(Cidr { base, prefix })
        } else {
            let base = Ipv4Addr::from_str(s)
                .expect("Некорректный формат ввода IP адреса");
            let prefix = 32_u8;
            Ok(Cidr { base, prefix })
        }
    }
}

impl std::fmt::Display for Cidr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.base, self.prefix)
    }
}