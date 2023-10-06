use std::env;

mod cidr;
mod cidr_error;
mod ipv4;
mod ipv4_error;
mod ipv4_mask;
mod ipv4_mask_error;
mod ipv4_network_range;

fn main() {
	let args: Vec<_> = env::args().collect();

	for i in 0..args.len() {
		println!("{}:{}", i, args[i]);
	}
}
