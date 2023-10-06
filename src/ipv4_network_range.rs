use crate::ipv4::IPv4;
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq)]
pub struct IPv4NetworkRange {
	begin: IPv4,
	end: IPv4,
}

impl IPv4NetworkRange {
	pub(crate) fn new(begin: IPv4, end: IPv4) -> IPv4NetworkRange {
		if begin.to_u32() > end.to_u32() {
			panic!("begin must be less than or equal to end");
		}

		IPv4NetworkRange { begin, end }
	}

	pub fn contain(&self, address: &IPv4) -> bool {
		self.begin.to_u32() <= address.to_u32() && address.to_u32() <= self.end.to_u32()
	}

	pub fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} - {}", self.begin, self.end)
	}

	pub fn begin(&self) -> &IPv4 {
		&self.begin
	}

	pub fn end(&self) -> &IPv4 {
		&self.end
	}
}

impl Debug for IPv4NetworkRange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for IPv4NetworkRange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

#[cfg(test)]
mod tests {
	use super::IPv4NetworkRange;
	use crate::ipv4::IPv4;
	use once_cell::sync::Lazy;
	static EXPECTED_BEGIN: Lazy<IPv4> = Lazy::new(|| IPv4::try_from("192.168.112.0").unwrap());
	static EXPECTED_END: Lazy<IPv4> = Lazy::new(|| IPv4::try_from("192.168.127.255").unwrap());

	fn fixture() -> IPv4NetworkRange {
		IPv4NetworkRange::new(*EXPECTED_BEGIN, *EXPECTED_END)
	}

	fn assert(actual: &IPv4NetworkRange, expected_begin: &IPv4, expected_end: &IPv4) {
		assert_eq!(&actual.begin, expected_begin);
		assert_eq!(&actual.end, expected_end);
	}

	#[test]
	fn new_test() {
		let actual = IPv4NetworkRange::new(*EXPECTED_BEGIN, *EXPECTED_END);
		assert(&actual, &EXPECTED_BEGIN, &EXPECTED_END);
	}

	#[test]
	fn contain_test() {
		let fixture = fixture();

		assert!(fixture.contain(&EXPECTED_BEGIN));
		assert!(fixture.contain(&EXPECTED_END));
		assert!(fixture.contain(&IPv4::try_from("192.168.121.101").unwrap()));

		let addr = IPv4::from(&EXPECTED_BEGIN.to_u32() - 1);
		assert!(!fixture.contain(&addr));

		let addr = IPv4::from(&EXPECTED_END.to_u32() + 1);
		assert!(!fixture.contain(&addr));
	}

	#[test]
	fn debug_test() {
		let fixture = fixture();
		assert_eq!(format!("{:?}", fixture), "192.168.112.0 - 192.168.127.255");
	}

	#[test]
	fn display_test() {
		let fixture = fixture();
		assert_eq!(format!("{}", fixture), "192.168.112.0 - 192.168.127.255");
	}

	#[test]
	#[should_panic]
	fn new_test_panic() {
		let _ = IPv4NetworkRange::new(*EXPECTED_END, *EXPECTED_BEGIN);
	}

	#[test]
	fn begin_test() {
		let fix = fixture();
		assert_eq!(&fix.begin, fix.begin());
		assert_eq!(&fix.end, fix.end());
	}
}
