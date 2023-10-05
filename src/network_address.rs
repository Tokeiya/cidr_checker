use crate::ipv4::IPv4;
use crate::network_address_error::NetworkAddressError;
use crate::subnet_mask::SubnetMask;
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq)]
pub struct NetworkAddress(IPv4, SubnetMask);

impl NetworkAddress {
	pub fn new(address: IPv4, mask: SubnetMask) -> Result<NetworkAddress, NetworkAddressError> {
		todo!()
	}

	pub fn contains(&self, address: &IPv4) -> bool {
		todo!()
	}

	pub fn address(&self) -> &IPv4 {
		todo!()
	}

	pub fn subnet_mask(&self) -> &SubnetMask {
		todo!()
	}

	pub fn broadcast_address(&self) -> IPv4 {
		todo!()
	}

	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Debug for NetworkAddress {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for NetworkAddress {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use crate::ipv4::IPv4;
	use crate::network_address::NetworkAddress;
	use crate::network_address_error::NetworkAddressError;
	use crate::subnet_mask::SubnetMask;

	fn assert_error<T>(result: Result<T, NetworkAddressError>, expected: NetworkAddressError) {
		fn to_ordinal(e: NetworkAddressError) -> usize {
			match e {
				NetworkAddressError::InvalidAddress => 1,
			}
		}

		if let Err(e) = result {
			assert_eq!(to_ordinal(e), to_ordinal(expected));
		} else {
			unreachable!()
		}
	}
	#[test]
	fn new_test() {
		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			SubnetMask::new(24).unwrap(),
		)
		.unwrap();
		assert_eq!(fixture.0, IPv4::try_from("192.168.10.0").unwrap());
		assert_eq!(fixture.1, SubnetMask::new(24).unwrap());

		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.1").unwrap(),
			SubnetMask::new(24).unwrap(),
		);

		assert_error(fixture, NetworkAddressError::InvalidAddress);
	}

	#[test]
	fn debug_test() {
		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			SubnetMask::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(format!("{:?}", fixture), "192.168.10.0/24")
	}

	#[test]
	fn display_test() {
		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			SubnetMask::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(format!("{}", fixture), "192.168.10.0/24")
	}

	#[test]
	fn address_test() {
		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			SubnetMask::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(fixture.address(), &fixture.0);
	}

	#[test]
	fn subnet_mask_test() {
		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			SubnetMask::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(fixture.subnet_mask(), &fixture.1);
	}

	#[test]
	fn contain_test() {
		let fixture = NetworkAddress::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			SubnetMask::new(24).unwrap(),
		)
		.unwrap();

		let addr = IPv4::try_from("192.168.19.120").unwrap();

		assert!(fixture.contains(&addr));

		let addr = IPv4::try_from("192.168.10.120").unwrap();
		assert!(!fixture.contains(&addr));
	}
}
