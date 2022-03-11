mod stat;
mod ip_network_arg;

use crate::ip_network_arg::IpNetworkArg;
use crate::stat::*;

fn main() {
    let network = IpNetworkArg::new();
    println!("Start IP:\t{}\nPrefix:\t\t/{} ({}-bit)", network.addr, network.prefix, cidr_addr_count(network.prefix));
    println!();
    let addresses = network.to_vec();
    for v in addresses {
        println!("{}", v)
    }
}