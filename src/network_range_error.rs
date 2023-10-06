use std::fmt::{Debug, Display, Formatter};

pub enum NetworkRangeError {
	InvalidAddressOrMask,
}

impl NetworkRangeError {
	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let scr = match self {
			NetworkRangeError::InvalidAddressOrMask => "NetworkRangeError::InvalidAddressOrMask",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for NetworkRangeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for NetworkRangeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::network_range_error::NetworkRangeError;

	#[test]
	fn debug_test() {
		let fixture = NetworkRangeError::InvalidAddressOrMask;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "NetworkRangeError::InvalidAddressOrMask");
	}

	#[test]
	fn display_test() {
		let fixture = NetworkRangeError::InvalidAddressOrMask;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "NetworkRangeError::InvalidAddressOrMask");
	}
}
