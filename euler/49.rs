extern crate exscape;
use exscape::{is_prime, is_permutation};

fn main() {
	// Ugly, but very simple.
	for i in range(1000u, 10000 - 3330*2) {
		if i != 1487 && is_permutation(i, i + 3330) && is_permutation(i, i + 3330*2) && is_prime(i) && is_prime(i+3330) && is_prime(i+3330*2) {
			println!("Answer: {}{}{}", i, i+3330, i+3330*2);
		}
	}
}
