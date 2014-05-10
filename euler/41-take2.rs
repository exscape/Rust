extern crate exscape;
use exscape::is_prime;
use std::iter::AdditiveIterator;

fn main() {
	let num = &mut[7u,6,5,4,3,2,1];

	loop {
		// Reconstruct array into a number; e.g. 7 * 10^6 + 6 * 10^5 + 5 * 10^4 + 1 * 10^0... = 7654321
		let n = num.iter().enumerate().map(|(pos,&val)| val * std::num::pow(10u,num.len() - pos - 1)).sum();

		if is_prime(n) {
			println!("Answer: {}", n);
			break;
		}
		num.prev_permutation();
	}
}
