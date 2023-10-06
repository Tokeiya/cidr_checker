use std::fmt::{Debug, Display, Formatter};

use crate::cidr::Cidr;
use crate::cidr_error::CidrError;
use crate::ipv4::IPv4;

#[derive(Eq, PartialEq)]
pub struct NetworkRange(IPv4, Cidr);

impl NetworkRange {
	pub fn new(address: IPv4, subnet: Cidr) -> Result<NetworkRange, CidrError> {
		let tmp = subnet.network_address(&address);
		if tmp != address {
			Err(CidrError::InvalidAddressOrMask)
		} else {
			Ok(NetworkRange(address, subnet))
		}
	}

	pub fn contains(&self, address: &IPv4) -> bool {
		let tmp = self.1.network_address(address);
		tmp == self.0
	}

	pub fn address(&self) -> &IPv4 {
		&self.0
	}

	pub fn subnet_mask(&self) -> &Cidr {
		&self.1
	}

	pub fn broadcast_address(&self) -> IPv4 {
		self.1.broadcast_address(&self.0)
	}

	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.0, self.1)
	}
}

impl Debug for NetworkRange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for NetworkRange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::cidr::Cidr;
	use crate::cidr_error::CidrError;
	use crate::ipv4::IPv4;
	use crate::network_range::NetworkRange;

	fn assert_error<T>(result: Result<T, CidrError>, expected: CidrError) {
		fn to_ordinal(e: CidrError) -> usize {
			match e {
				CidrError::InvalidAddressOrMask => 1,
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
		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			Cidr::new(24).unwrap(),
		)
		.unwrap();
		assert_eq!(fixture.0, IPv4::try_from("192.168.10.0").unwrap());
		assert_eq!(fixture.1, Cidr::new(24).unwrap());

		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.1").unwrap(),
			Cidr::new(24).unwrap(),
		);

		assert_error(fixture, CidrError::InvalidAddressOrMask);
	}

	#[test]
	fn debug_test() {
		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			Cidr::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(format!("{:?}", fixture), "192.168.10.0/24")
	}

	#[test]
	fn display_test() {
		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			Cidr::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(format!("{}", fixture), "192.168.10.0/24")
	}

	#[test]
	fn address_test() {
		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			Cidr::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(fixture.address(), &fixture.0);
	}

	#[test]
	fn subnet_mask_test() {
		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			Cidr::new(24).unwrap(),
		)
		.unwrap();

		assert_eq!(fixture.subnet_mask(), &fixture.1);
	}

	#[test]
	fn contain_test() {
		let fixture = NetworkRange::new(
			IPv4::try_from("192.168.10.0").unwrap(),
			Cidr::new(24).unwrap(),
		)
		.unwrap();

		let addr = IPv4::try_from("192.168.10.120").unwrap();

		assert!(fixture.contains(&addr));

		let addr = IPv4::try_from("192.168.11.120").unwrap();
		assert!(!fixture.contains(&addr));
	}
}
