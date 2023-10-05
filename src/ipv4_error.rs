use std::fmt::{Debug, Display, Formatter};

pub enum Ipv4Error {
	IndexOutOfRange,
	InvalidFormat,
}

impl Ipv4Error {
	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let scr = match self {
			Ipv4Error::IndexOutOfRange => "Ipv4Error::IndexOutOfRange",
			Ipv4Error::InvalidFormat => "Ipv4Error::InvalidFormat",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for Ipv4Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for Ipv4Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::ipv4_error::Ipv4Error;

	#[test]
	pub fn debug_test() {
		let actual = format!("{:?}", Ipv4Error::IndexOutOfRange);
		assert_eq!(actual, "Ipv4Error::IndexOutOfRange");

		let actual = format!("{:?}", Ipv4Error::InvalidFormat);
		assert_eq!(actual, "Ipv4Error::InvalidFormat");
	}

	#[test]

	pub fn display_test() {
		let actual = format!("{:}", Ipv4Error::IndexOutOfRange);
		assert_eq!(actual, "Ipv4Error::IndexOutOfRange");

		let actual = format!("{:}", Ipv4Error::InvalidFormat);
		assert_eq!(actual, "Ipv4Error::InvalidFormat");
	}
}
