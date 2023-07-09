use anyhow::{bail, Result};
use ipnetwork::Ipv4Network;
use itertools::Itertools;
use std::net::Ipv4Addr;

pub trait Vlsm {
    fn vlsm(self, sizes: &Vec<u32>, include_net_brd_addr: bool) -> Result<Vec<Ipv4Network>>;
}

impl Vlsm for Ipv4Network {
    fn vlsm(self, sizes: &Vec<u32>, include_net_brd_addr: bool) -> Result<Vec<Ipv4Network>> {
        let mut subnets: Vec<Ipv4Network> = Vec::new();

        let mut current_address = self.network();
        let mut total_hosts = 0u32;
        for hosts in sizes.iter().sorted().rev() {
            let hosts = if include_net_brd_addr {
                hosts + 2
            } else {
                hosts.clone()
            };
            let prefix: u8 = 32u8
                .checked_sub(hosts.next_power_of_two().ilog2() as u8)
                .expect("Can't ask for more than 2^32 - 1 hosts");
            total_hosts += 1 << (32 - prefix);
            let current_subnet = Ipv4Network::new(current_address, prefix)?;
            if total_hosts > self.size() {
                bail!(
                    "Not enough room in subnet {} to fit {} hosts (inc. net/brd addresses)",
                    self,
                    total_hosts
                );
            }
            subnets.push(current_subnet);
            current_address = Ipv4Addr::from(u32::from(current_subnet.broadcast()) + 1);
        }

        Ok(subnets)
    }
}
