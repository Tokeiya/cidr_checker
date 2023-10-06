use std::fmt::{Debug, Display, Formatter};

type FmtResult = std::fmt::Result;

pub enum IPv4MaskError {
	CidrOutOfRange,
	FormatError,
	InvalidSubnetMask,
}

impl IPv4MaskError {
	fn format(&self, f: &mut Formatter<'_>) -> FmtResult {
		let scr = match self {
			IPv4MaskError::CidrOutOfRange => "IPv4MaskError::CidrOutOfRange",
			IPv4MaskError::FormatError => "IPv4MaskError::FormatError",
			IPv4MaskError::InvalidSubnetMask => "IPv4MaskError::InvalidSubnetMask",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for IPv4MaskError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for IPv4MaskError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::ipv4_mask_error::IPv4MaskError;

	#[test]
	fn debug_test() {
		let fixture = IPv4MaskError::CidrOutOfRange;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "IPv4MaskError::CidrOutOfRange");

		let fixture = IPv4MaskError::FormatError;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "IPv4MaskError::FormatError");

		let fixture = IPv4MaskError::InvalidSubnetMask;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "IPv4MaskError::InvalidSubnetMask");
	}

	#[test]
	fn display_test() {
		let fixture = IPv4MaskError::CidrOutOfRange;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "IPv4MaskError::CidrOutOfRange");

		let fixture = IPv4MaskError::FormatError;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "IPv4MaskError::FormatError");

		let fixture = IPv4MaskError::InvalidSubnetMask;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "IPv4MaskError::InvalidSubnetMask");
	}
}
