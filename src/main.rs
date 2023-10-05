use crate::ipv4::IPv4;

mod ipv4;
mod ipv4_error;

fn main() {
	let ip = IPv4::from(0xC0_A8_0A_66);
	println!("{}", ip);
	println!("{:?}", ip);
}
