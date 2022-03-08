use std::net::Ipv4Addr;
use std::env::args;
use clap::{Arg, App};

fn main() {
    let app = App::new("test1")
        .version("0.0.1")
        .about("Do something")
        .author("quantmad");



    let a : u8 = 10; let b : u8 = 88; let c : u8 = 2; let d : u8 = 3;
    let n = ip4tou32(a, b, c, d);
    println!("{}.{}.{}.{} = {}", a, b, c, d, n);
    let i = u32toip4(n).octets();
    println!("{} = {}.{}.{}.{}", n, i[0], i[1], i[2], i[3]);
    println!("{}", cidr_addr_count(24));
}

fn cidr_addr_count(prefix:u8) -> u32 {
    if prefix > 32 {
        panic!("CIDR prefix can't be greater than 32")
    }
    2_u32.pow((32 - prefix) as u32)
}

fn ip4tou32(a:u8, b:u8, c:u8, d:u8) -> u32 {
    u32::from(Ipv4Addr::new(a, b, c, d))
}

fn u32toip4(num:u32) -> Ipv4Addr {
    Ipv4Addr::from(num)
}