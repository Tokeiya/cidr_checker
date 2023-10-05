use std::fmt::{Debug, Display, Formatter};

pub enum NetworkAddressError {
	InvalidAddress,
}

impl NetworkAddressError {
	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let scr = match self {
			NetworkAddressError::InvalidAddress => "NetworkAddressError::InvalidAddress",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for NetworkAddressError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for NetworkAddressError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::network_address_error::NetworkAddressError;

	#[test]
	fn debug_test() {
		let fixture = NetworkAddressError::InvalidAddress;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "NetworkAddressError::InvalidAddress");
	}

	#[test]
	fn display_test() {
		let fixture = NetworkAddressError::InvalidAddress;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "NetworkAddressError::InvalidAddress");
	}
}
