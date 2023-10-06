use std::fmt::{Debug, Display, Formatter};

use once_cell::sync::Lazy;

use crate::ipv4::IPv4;
use crate::ipv4_mask_error::IPv4MaskError;

static TABLE: Lazy<[IPv4; 32]> = Lazy::new(|| {
	[
		IPv4::from(0x80000000u32),
		IPv4::from(0xc0000000u32),
		IPv4::from(0xe0000000u32),
		IPv4::from(0xf0000000u32),
		IPv4::from(0xf8000000u32),
		IPv4::from(0xfc000000u32),
		IPv4::from(0xfe000000u32),
		IPv4::from(0xff000000u32),
		IPv4::from(0xff800000u32),
		IPv4::from(0xffc00000u32),
		IPv4::from(0xffe00000u32),
		IPv4::from(0xfff00000u32),
		IPv4::from(0xfff80000u32),
		IPv4::from(0xfffc0000u32),
		IPv4::from(0xfffe0000u32),
		IPv4::from(0xffff0000u32),
		IPv4::from(0xffff8000u32),
		IPv4::from(0xffffc000u32),
		IPv4::from(0xffffe000u32),
		IPv4::from(0xfffff000u32),
		IPv4::from(0xfffff800u32),
		IPv4::from(0xfffffc00u32),
		IPv4::from(0xfffffe00u32),
		IPv4::from(0xffffff00u32),
		IPv4::from(0xffffff80u32),
		IPv4::from(0xffffffc0u32),
		IPv4::from(0xffffffe0u32),
		IPv4::from(0xfffffff0u32),
		IPv4::from(0xfffffff8u32),
		IPv4::from(0xfffffffcu32),
		IPv4::from(0xfffffffeu32),
		IPv4::from(0xffffffffu32),
	]
});

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct IPv4Mask(IPv4, u8);

impl IPv4Mask {
	pub fn new(cidr: u8) -> Result<Self, IPv4MaskError> {
		if cidr > 0 && cidr <= 32 {
			Ok(IPv4Mask(TABLE[(cidr - 1) as usize], cidr))
		} else {
			Err(IPv4MaskError::CidrOutOfRange)
		}
	}

	pub fn cidr(&self) -> u8 {
		self.1
	}

	pub fn mask_address(&self) -> &IPv4 {
		&self.0
	}

	pub fn network_address(&self, ip: &IPv4) -> IPv4 {
		let addr = self.0.to_u32() & ip.to_u32();
		IPv4::from(addr)
	}

	pub fn broadcast_address(&self, ip: &IPv4) -> IPv4 {
		let mut addr = self.0.to_u32() & ip.to_u32();
		addr |= !self.0.to_u32();
		IPv4::from(addr)
	}

	pub fn range(&self) -> (IPv4, IPv4) {
		todo!()
	}

	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "/{}", self.1)
	}
}

impl TryFrom<&IPv4> for IPv4Mask {
	type Error = IPv4MaskError;

	fn try_from(value: &IPv4) -> Result<Self, Self::Error> {
		let mut cidr = 1u8;

		for expected in TABLE.iter() {
			if value == expected {
				return Ok(IPv4Mask::new(cidr).unwrap());
			}

			cidr += 1;
		}

		Err(IPv4MaskError::InvalidSubnetMask)
	}
}

impl Debug for IPv4Mask {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for IPv4Mask {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use once_cell::sync::Lazy;

	use crate::ipv4::IPv4;
	use crate::ipv4_mask::IPv4Mask;
	use crate::ipv4_mask_error::IPv4MaskError;

	static MASK: Lazy<[IPv4; 32]> = Lazy::new(|| {
		[
			IPv4::try_from("128.0.0.0").unwrap(),
			IPv4::try_from("192.0.0.0").unwrap(),
			IPv4::try_from("224.0.0.0").unwrap(),
			IPv4::try_from("240.0.0.0").unwrap(),
			IPv4::try_from("248.0.0.0").unwrap(),
			IPv4::try_from("252.0.0.0").unwrap(),
			IPv4::try_from("254.0.0.0").unwrap(),
			IPv4::try_from("255.0.0.0").unwrap(),
			IPv4::try_from("255.128.0.0").unwrap(),
			IPv4::try_from("255.192.0.0").unwrap(),
			IPv4::try_from("255.224.0.0").unwrap(),
			IPv4::try_from("255.240.0.0").unwrap(),
			IPv4::try_from("255.248.0.0").unwrap(),
			IPv4::try_from("255.252.0.0").unwrap(),
			IPv4::try_from("255.254.0.0").unwrap(),
			IPv4::try_from("255.255.0.0").unwrap(),
			IPv4::try_from("255.255.128.0").unwrap(),
			IPv4::try_from("255.255.192.0").unwrap(),
			IPv4::try_from("255.255.224.0").unwrap(),
			IPv4::try_from("255.255.240.0").unwrap(),
			IPv4::try_from("255.255.248.0").unwrap(),
			IPv4::try_from("255.255.252.0").unwrap(),
			IPv4::try_from("255.255.254.0").unwrap(),
			IPv4::try_from("255.255.255.0").unwrap(),
			IPv4::try_from("255.255.255.128").unwrap(),
			IPv4::try_from("255.255.255.192").unwrap(),
			IPv4::try_from("255.255.255.224").unwrap(),
			IPv4::try_from("255.255.255.240").unwrap(),
			IPv4::try_from("255.255.255.248").unwrap(),
			IPv4::try_from("255.255.255.252").unwrap(),
			IPv4::try_from("255.255.255.254").unwrap(),
			IPv4::try_from("255.255.255.255").unwrap(),
		]
	});

	fn assert_error<T>(actual: Result<T, IPv4MaskError>, expected: IPv4MaskError) {
		fn to_ordinal(e: IPv4MaskError) -> usize {
			match e {
				IPv4MaskError::CidrOutOfRange => 1,
				IPv4MaskError::FormatError => 2,
				IPv4MaskError::InvalidSubnetMask => 3,
			}
		}

		if let Err(err) = actual {
			assert_eq!(to_ordinal(err), to_ordinal(expected))
		} else {
			unreachable!()
		}
	}

	#[test]
	fn new_test() {
		for i in 1..=32 {
			let actual = IPv4Mask::new(i);
			assert!(actual.is_ok());
			assert_eq!(actual.unwrap().cidr(), i);
		}

		assert_error(IPv4Mask::new(33), IPv4MaskError::CidrOutOfRange);
		assert_error(IPv4Mask::new(0), IPv4MaskError::CidrOutOfRange);
	}

	#[test]
	fn from_subnet_mask_test() {
		let mut cidr = 1;

		for addr in MASK.iter() {
			let fixture = IPv4Mask::try_from(addr).unwrap();

			assert_eq!(cidr, fixture.cidr());
			cidr += 1;
		}

		let fixture = IPv4Mask::try_from(&IPv4::try_from("128.1.0.1").unwrap());
		assert_error(fixture, IPv4MaskError::InvalidSubnetMask);
	}

	#[test]
	fn debug_test() {
		for i in 1u8..=32 {
			let fixture = IPv4Mask::new(i).unwrap();
			assert_eq!(format!("{:?}", fixture), format!("/{}", i))
		}
	}

	#[test]
	fn display_test() {
		for i in 1u8..=32 {
			let fixture = IPv4Mask::new(i).unwrap();
			assert_eq!(format!("{:}", fixture), format!("/{}", i))
		}
	}

	#[test]
	fn subnet_mask_test() {
		for i in 1u8..=32 {
			let fixture = IPv4Mask::new(i).unwrap();
			let expected = &MASK[(i - 1) as usize];

			assert_eq!(fixture.mask_address(), expected)
		}
	}

	#[test]
	fn network_address_test() {
		let mask = IPv4Mask::try_from(&IPv4::try_from("255.255.255.0").unwrap()).unwrap();
		let addr = IPv4::try_from("192.168.10.102").unwrap();

		let actual = mask.network_address(&addr);

		assert_eq!(actual, IPv4::try_from("192.168.10.0").unwrap())
	}

	#[test]
	fn broadcast_address_test() {
		let mask = IPv4Mask::try_from(&IPv4::try_from("255.255.255.0").unwrap()).unwrap();
		let addr = IPv4::try_from("192.168.10.102").unwrap();

		let actual = mask.broadcast_address(&addr);

		assert_eq!(actual, IPv4::try_from("192.168.10.255").unwrap())
	}

	#[test]
	fn range_test() {
		todo!();
	}

	#[test]
	fn eq_test() {
		let a = IPv4Mask::new(1).unwrap();
		let b = IPv4Mask::new(1).unwrap();

		assert!(a == b);
		assert!(!(a != b));
	}

	#[test]
	fn ne_test() {
		let a = IPv4Mask::new(1).unwrap();
		let b = IPv4Mask::new(2).unwrap();

		assert!(!(a == b));
		assert!(a != b);
	}
}
