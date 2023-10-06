use crate::cidr::Cidr;
use crate::ipv4::IPv4;
use crate::network_range::NetworkRange;

mod cidr;
mod cidr_error;
mod ipv4;
mod ipv4_error;
mod network_range;
mod subnet_error;

fn main() {
	let network = NetworkRange::new(
		IPv4::try_from("192.168.112.0").unwrap(),
		Cidr::new(20).unwrap(),
	)
	.unwrap();

	let verify_target = IPv4::try_from("192.168.121.101").unwrap();

	println!("NetworkRange:{:?}", network);
	println!("NetworkAddress:{}", network.address());
	println!("SubnetMask:{}", network.subnet_mask().mask_address());
	println!("VerifyTarget:{}", verify_target);
	println!("IsContain:{}", network.contains(&verify_target));
}
