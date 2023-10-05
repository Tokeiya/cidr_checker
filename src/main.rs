mod ipv4;
mod ipv4_error;

fn main() {
	let mut piv = 0xFFu32;

	for i in 0..4u32 {
		println!("{:x}", piv);
		piv <<= 8;
	}
}
