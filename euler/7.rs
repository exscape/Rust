/* 2014-04-17, Rust 0.11-pre */

extern crate exscape;
use exscape::is_prime;

fn main() {
	let mut count = 0;
	let mut n = 2;
	loop {
		if is_prime(n) {
			count += 1;
			if count == 10001 { break; }
		}
		n += 1;
	}

	println!("Answer: 10001st prime is {}", n);
}
