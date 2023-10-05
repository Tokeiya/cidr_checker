use std::fmt::{Debug, Display, Formatter};

type FmtResult = std::fmt::Result;

pub enum SubnetMaskError {
	CidrOutOfRange,
	FormatError,
	InvalidSubnetMask,
}

impl SubnetMaskError {
	fn format(&self, f: &mut Formatter<'_>) -> FmtResult {
		let scr = match self {
			SubnetMaskError::CidrOutOfRange => "SubnetMaskError::CidrOutOfRange",
			SubnetMaskError::FormatError => "SubnetMaskError::FormatError",
			SubnetMaskError::InvalidSubnetMask => "SubnetMaskError::InvalidSubnetMask",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for SubnetMaskError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for SubnetMaskError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::subnet_mask_error::SubnetMaskError;

	#[test]
	fn debug_test() {
		let fixture = SubnetMaskError::CidrOutOfRange;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "SubnetMaskError::CidrOutOfRange");

		let fixture = SubnetMaskError::FormatError;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "SubnetMaskError::FormatError");

		let fixture = SubnetMaskError::InvalidSubnetMask;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "SubnetMaskError::InvalidSubnetMask");
	}

	#[test]
	fn display_test() {
		let fixture = SubnetMaskError::CidrOutOfRange;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "SubnetMaskError::CidrOutOfRange");

		let fixture = SubnetMaskError::FormatError;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "SubnetMaskError::FormatError");

		let fixture = SubnetMaskError::InvalidSubnetMask;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "SubnetMaskError::InvalidSubnetMask");
	}
}
