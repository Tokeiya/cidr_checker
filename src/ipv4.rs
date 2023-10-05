use crate::ipv4_error::Ipv4Error;
use once_cell::sync::Lazy;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;

struct Factor(u32, u32);

static OFFSET_TABLE: Lazy<[Factor; 4]> = Lazy::new(|| {
	[
		Factor(0xff_00_00_00, 24u32),
		Factor(0xff_00_00, 16u32),
		Factor(0xff_00, 8u32),
		Factor(0xff, 0u32),
	]
});

pub struct Ipv4(u32);

impl Ipv4 {
	//    pub fn new(source:&str)
}
