mod stat;

use clap::{App};
use crate::stat::*;

fn main() {
    let app = App::new("rcidr")
        .version("v0.0.1")
        .about("About:\tDo something")
        .author("Author:\tQuantMad");

    let _ = app.get_matches();

    let a : u8 = 10; let b : u8 = 88; let c : u8 = 2; let d : u8 = 3;
    let n = ip4_to_u32(a, b, c, d);
    println!("{}.{}.{}.{} = {}", a, b, c, d, n);
    let i = u32_to_ip4(n).octets();
    println!("{} = {}.{}.{}.{}", n, i[0], i[1], i[2], i[3]);
    println!("{}",  cidr_addr_count(24));
}