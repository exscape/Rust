extern crate exscape;
use exscape::factor;

fn main() {
	let num = 600851475143;
	let factors = factor(num);
	println!("Biggest factor: {}", factors.last().unwrap());
}
