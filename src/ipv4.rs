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

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct IPv4(u32);

impl IPv4 {
	pub fn fragment(&self, index: usize) -> Result<u8, Ipv4Error> {
		if index > 3 {
			Err(Ipv4Error::IndexOutOfRange)
		} else {
			let Factor(offset, shift) = OFFSET_TABLE[index];
			Ok(((self.0 & offset) >> shift) as u8)
		}
	}

	pub fn to_u32(&self) -> u32 {
		self.0
	}

	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for i in 0..4 {
			write!(f, "{}", self.fragment(i).unwrap())?;
			if i < 3 {
				write!(f, ".")?;
			}
		}
		Ok(())
	}
}

impl From<u32> for IPv4 {
	fn from(value: u32) -> Self {
		IPv4(value)
	}
}

impl TryFrom<&[u8]> for IPv4 {
	type Error = Ipv4Error;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		if value.len() != 4 {
			Err(Ipv4Error::InvalidArrayLength)
		} else {
			let mut result = 0u32;
			for i in 0..value.len() {
				let byte = value[i];
				let Factor(_, shift) = OFFSET_TABLE[i];

				result |= (byte as u32) << shift;
			}

			Ok(IPv4(result))
		}
	}
}

impl TryFrom<&str> for IPv4 {
	type Error = Ipv4Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let mut cnt = 0;
		let mut accum: u32 = 0;

		for elem in value.split(".") {
			if cnt >= 4 {
				return Err(Ipv4Error::InvalidFormat);
			}

			let Factor(_, shift) = OFFSET_TABLE[cnt];

			let tmp = elem.parse::<u8>().map_err(|_| Ipv4Error::InvalidValue)? as u32;
			accum |= tmp << shift;

			cnt += 1;
		}

		if cnt != 4 {
			Err(Ipv4Error::InvalidFormat)
		} else {
			Ok(IPv4(accum))
		}
	}
}

impl Debug for IPv4 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for IPv4 {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::ipv4::IPv4;
	use crate::ipv4_error::Ipv4Error;

	fn assert(actual: IPv4, expected: &[u8; 4]) {
		for i in 0..4usize {
			assert_eq!(actual.fragment(i).unwrap(), expected[i])
		}
	}

	fn assert_err<T>(actual: Result<T, Ipv4Error>, expected: Ipv4Error) {
		fn to_ordinal(e: Ipv4Error) -> usize {
			match e {
				Ipv4Error::IndexOutOfRange => 1,
				Ipv4Error::InvalidFormat => 2,
				Ipv4Error::InvalidArrayLength => 3,
				Ipv4Error::InvalidValue => 4,
			}
		}

		if let Err(err) = actual {
			assert_eq!(to_ordinal(err), to_ordinal(expected));
		} else {
			unreachable!()
		}
	}

	#[test]
	fn from_u32_test() {
		let fixture = IPv4::from(0xC0_A8_0A_66);

		for i in 0..4 {
			print!("{}.", fixture.fragment(i).unwrap());
		}

		println!();
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
		assert_eq!(format!("{:?}", IPv4::from(0xC0_A8_0A_66)), "192.168.10.102");
	}

	#[test]
	fn display_test() {
		let actual = format!("{}", IPv4::from(0xC0_A8_0A_66));
		assert_eq!(format!("{}", actual), "192.168.10.102");
	}

	#[test]
	fn fragment_test() {
		let fixture = IPv4::from(0xC0_A8_0A_66);
		assert_eq!(fixture.fragment(0).unwrap(), 192u8);
		assert_eq!(fixture.fragment(1).unwrap(), 168u8);
		assert_eq!(fixture.fragment(2).unwrap(), 10u8);
		assert_eq!(fixture.fragment(3).unwrap(), 102u8);

		assert_err(fixture.fragment(4), Ipv4Error::IndexOutOfRange);
	}

	#[test]
	fn to_u32_test() {
		let fixture = IPv4::from(0xC0_A8_0A_66);
		assert_eq!(fixture.to_u32(), fixture.0)
	}
}
