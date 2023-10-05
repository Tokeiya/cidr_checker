use crate::ipv4::IPv4;
use crate::subnet_mask::SubnetMask;
use std::net::IpAddr;

mod ipv4;
mod ipv4_error;
mod subnet_mask;
mod subnet_mask_error;

fn main() {
	println!("{}", !0);
}
