extern crate exscape;
use exscape::{trunc_left, trunc_right, is_prime};
use std::iter::count;

fn main() {
	let mut found = 0;
	let mut sum = 0;

	'outer: for candidate in count(11u, 2) {
		let mut left = candidate;
		let mut right = candidate;
		while left > 0 && right > 0 {
			if !is_prime(left) || !is_prime(right) { continue 'outer; }
			left = trunc_left(left);
			right = trunc_right(right);
		}

		// If we get here, the loop completed and all truncations were prime
		sum += candidate;
		found += 1;

		// The problem states that there are 11 such numbers
		if found == 11 { break; }
	}

	println!("Answer: {}", sum);
}
