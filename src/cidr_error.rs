use std::fmt::{Debug, Display, Formatter};

pub enum CidrError {
	InvalidAddressOrMask,
}

impl CidrError {
	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let scr = match self {
			CidrError::InvalidAddressOrMask => "CidrError::InvalidAddressOrMask",
		};

		write!(f, "{}", scr)
	}
}

impl Debug for CidrError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for CidrError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::cidr_error::CidrError;
	#[test]
	fn debug_test() {
		let fixture = CidrError::InvalidAddressOrMask;
		let actual = format!("{:?}", fixture);
		assert_eq!(actual, "CidrError::InvalidAddressOrMask");
	}

	#[test]
	fn display_test() {
		let fixture = CidrError::InvalidAddressOrMask;
		let actual = format!("{:}", fixture);
		assert_eq!(actual, "CidrError::InvalidAddressOrMask");
	}
}
