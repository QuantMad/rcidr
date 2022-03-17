use std::net::Ipv4Addr;
use clap::Parser;
use std::str::FromStr;
use std::fmt::{Display, Formatter};

const MAX_PREFIX: u8 = 32_u8;

#[derive(Parser, Debug)]
pub struct Cidr {
    pub base: Ipv4Addr,
    pub prefix: u8,
}

impl FromStr for Cidr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            panic!("Отсутствует аргумент флага")
        }

        let pare = s.split('/').collect::<Vec<&str>>();

        let base = Ipv4Addr::from_str(pare[0])
            .expect(&*format!("Некорректный формат ввода IP адреса: {}", pare[0]));

        let prefix = if pare.len() == 2 {
            u8::from_str(pare[1])
                .expect(&*format!("Некорректный формат ввода префикса: {}", pare[1]))
        } else {
            MAX_PREFIX
        };

        if prefix > MAX_PREFIX {
            panic!("Префикс \"{prefix}\" больше максимально возможного: \"{MAX_PREFIX}\"")
        }

        Ok(Cidr{base, prefix})
    }
}

impl Display for Cidr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.base, self.prefix)
    }
}

impl Cidr {
    pub fn to_vec(&self) -> Vec<Ipv4Addr> {
        let mut result = Vec::<Ipv4Addr>::new();
        for i in 0..addr_count(self.prefix) {
            let dig = u32::from(self.base) + i;
            result.push(Ipv4Addr::from(u32::from(dig)));
        }

        return result;
    }
}

pub fn addr_count(prefix:u8) -> u32 {
    2_u32.pow((MAX_PREFIX - prefix) as u32)
}