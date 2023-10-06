use crate::ipv4::IPv4;
use crate::network_range::NetworkRange;
use crate::subnet::Subnet;

mod ipv4;
mod ipv4_error;
mod network_range;
mod network_range_error;
mod subnet;
mod subnet_error;

fn main() {
	let network = NetworkRange::new(
		IPv4::try_from("192.168.112.0").unwrap(),
		Subnet::new(20).unwrap(),
	)
	  .unwrap();

	let verify_target = IPv4::try_from("192.168.121.101").unwrap();

	println!("NetworkRange:{:?}", network);
	println!("SubnetMask:{}", network.subnet_mask().subnet_mask());
	println!("VerifyTarget:{}", verify_target);
	println!("IsContain:{}", network.contains(&verify_target));
}
