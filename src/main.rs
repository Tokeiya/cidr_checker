use crate::ipv4::IPv4;
use crate::network_address::NetworkAddress;
use crate::subnet_mask::SubnetMask;

mod ipv4;
mod ipv4_error;
mod network_address;
mod network_address_error;
mod subnet_mask;
mod subnet_mask_error;

fn main() {
	let fixture = NetworkAddress::new(
		IPv4::try_from("192.168.10.0").unwrap(),
		SubnetMask::new(24).unwrap(),
	)
	.unwrap();

	let addr = IPv4::try_from("192.168.10.120").unwrap();

	assert!(fixture.contains(&addr));

	let addr = IPv4::try_from("192.168.10.120").unwrap();
	assert!(!fixture.contains(&addr));
}
