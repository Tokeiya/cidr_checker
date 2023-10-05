use crate::ipv4_error::Ipv4Error;
use once_cell::sync::Lazy;
use std::fmt::{Debug, Display, Formatter};

struct Factor(u32, u32);

static OFFSET_TABLE: Lazy<[Factor; 4]> = Lazy::new(|| {
	[
		Factor(0xff_00_00_00, 24u32),
		Factor(0xff_00_00, 16u32),
		Factor(0xff_00, 8u32),
		Factor(0xff, 0u32),
	]
});

pub struct IPv4(u32);

impl IPv4 {
	pub fn fragment(&self, index: usize) -> u8 {
		todo!()
	}
}

impl From<u32> for IPv4 {
	fn from(value: u32) -> Self {
		todo!()
	}
}

impl TryFrom<&[u8]> for IPv4 {
	type Error = Ipv4Error;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl TryFrom<&str> for IPv4 {
	type Error = Ipv4Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl Debug for IPv4 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for IPv4 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use crate::ipv4::IPv4;
	use crate::ipv4_error::Ipv4Error;

	fn assert(actual: IPv4, expected: &[u8; 4]) {
		for i in 0..4usize {
			assert_eq!(actual.fragment(i), expected[i])
		}
	}

	fn assert_err(actual: Result<IPv4, Ipv4Error>, expected: Ipv4Error) {
		fn to_ordinal(e: Ipv4Error) -> usize {
			match e {
				Ipv4Error::IndexOutOfRange => 1,
				Ipv4Error::InvalidFormat => 2,
				Ipv4Error::InvalidArrayLength => 3,
				Ipv4Error::InvalidValue => 4,
			}
		}

		let err = actual.unwrap_err();
		assert_eq!(to_ordinal(err), to_ordinal(expected))
	}

	#[test]
	fn from_u32_test() {
		let fixture = IPv4::from(0x66_0A_A8_C0_u32);
		assert(fixture, &[192u8, 168u8, 10u8, 102u8])
	}

	#[test]
	fn try_from_str_test() {
		let fixture = IPv4::try_from("192.168.10.102").unwrap();
		assert(fixture, &[192u8, 168u8, 10u8, 102u8]);

		assert_err(IPv4::try_from("192.168.10.256"), Ipv4Error::InvalidValue);
		assert_err(IPv4::try_from("192.168.256.10"), Ipv4Error::InvalidValue);
		assert_err(IPv4::try_from("192.256.10.102"), Ipv4Error::InvalidValue);
		assert_err(IPv4::try_from("256.168.10.102"), Ipv4Error::InvalidValue);

		assert_err(IPv4::try_from("192.168.10"), Ipv4Error::InvalidFormat);
		assert_err(
			IPv4::try_from("192.168.10.10.102"),
			Ipv4Error::InvalidFormat,
		);
	}

	#[test]
	fn try_from_u8_slice_test() {
		let fixture = IPv4::try_from(&[192u8, 168u8, 10u8, 102u8][..]).unwrap();
		assert(fixture, &[192u8, 168u8, 10u8, 102u8]);

		assert_err(
			IPv4::try_from(&[192u8, 168u8, 10u8][..]),
			Ipv4Error::InvalidArrayLength,
		);
		assert_err(
			IPv4::try_from(&[192u8, 168u8, 10u8, 102u8, 0u8][..]),
			Ipv4Error::InvalidArrayLength,
		);
	}

	#[test]
	fn debug_test() {
		let actual = format!("{:?}", IPv4::from(0x66_0A_A8_C0_u32));
		assert_eq!(format!("{:?}", actual), "192.168.10.102");
	}

	#[test]
	fn display_test() {
		let actual = format!("{:?}", IPv4::from(0x66_0A_A8_C0_u32));
		assert_eq!(format!("{}", actual), "192.168.10.102");
	}

	#[test]
	fn fragment_test() {
		let fixture = IPv4::from(0x66_0A_A8_C0_u32);
		assert_eq!(fixture.fragment(0), 192u8);
		assert_eq!(fixture.fragment(1), 168u8);
		assert_eq!(fixture.fragment(2), 10u8);
		assert_eq!(fixture.fragment(3), 102u8);
	}
}
