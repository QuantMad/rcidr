use std::net::Ipv4Addr;

pub fn cidr_addr_count(prefix:u8) -> u32 {
    if prefix > 32 {
        panic!("CIDR prefix can't be greater than 32")
    }
    2_u32.pow((32 - prefix) as u32)
}

/*pub fn octets_to_u32(a:u8, b:u8, c:u8, d:u8) -> u32 {
    u32::from(Ipv4Addr::new(a, b, c, d))
}*/

pub fn ip4_to_u32(addr:Ipv4Addr) -> u32 {
    u32::from(addr)
}

pub fn u32_to_ip4(num:u32) -> Ipv4Addr {
    Ipv4Addr::from(num)
}