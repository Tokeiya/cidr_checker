use std::env;

mod cidr;
mod cidr_error;
mod command_data;
mod ipv4;
mod ipv4_error;
mod ipv4_mask;
mod ipv4_mask_error;
mod ipv4_network_range;

//-c 192.168.10.0/24
//-s 255.255.255.0(with -n
//-a 192.168.19.15
//-n 192.168.10.0 (With -s
fn main() {
	let args: Vec<_> = env::args().collect();
	let a: Vec<_> = env::args().collect();

	for elem in args {
		println!("{}", elem)
	}

	for i in 0..a.len() {
		println!("{}:{}", i, a[i])
	}
}

fn hoge<'a, T: Iterator<Item = &'a String>>(scr: &mut T) {
	for elem in scr {
		println!("{}", elem)
	}
}
