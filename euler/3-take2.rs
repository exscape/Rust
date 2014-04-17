extern crate exscape;
use exscape::factor;

fn main() {
	let num = 18446744073709551613;
	let factors = factor(num);
	println!("{}: {:?}", num, factors);
	println!("Biggest factor: {}", factors.last().unwrap());
}
