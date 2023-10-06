use std::fmt::{Debug, Display, Formatter};

type FmtResult = std::fmt::Result;

pub enum SubnetError {
	CidrOutOfRange,
	FormatError,
	InvalidSubnetMask,
}

impl SubnetError {
	fn format(&self, f: &mut Formatter<'_>) -> FmtResult {
		let scr = match self {
			SubnetError::CidrOutOfRange => "SubnetError::CidrOutOfRange",
			SubnetError::FormatError => "SubnetError::FormatError",
			SubnetError::InvalidSubnetMask => "SubnetError::InvalidSubnetMask",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for SubnetError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for SubnetError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::subnet_error::SubnetError;

	#[test]
	fn debug_test() {
		let fixture = SubnetError::CidrOutOfRange;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "SubnetError::CidrOutOfRange");

		let fixture = SubnetError::FormatError;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "SubnetError::FormatError");

		let fixture = SubnetError::InvalidSubnetMask;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "SubnetError::InvalidSubnetMask");
	}

	#[test]
	fn display_test() {
		let fixture = SubnetError::CidrOutOfRange;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "SubnetError::CidrOutOfRange");

		let fixture = SubnetError::FormatError;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "SubnetError::FormatError");

		let fixture = SubnetError::InvalidSubnetMask;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "SubnetError::InvalidSubnetMask");
	}
}
