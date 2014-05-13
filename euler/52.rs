extern crate exscape;
use exscape::is_permutation;
use std::iter::{count, range_inclusive};

fn main() {
	'outer: for n in count(1u, 1) { // I didn't bother with a proper lower limit
		for i in range_inclusive(2u,6) {
			if !is_permutation(n, i*n) {
				continue 'outer;
			}
		}

		println!("Answer: {}", n);
		break;
	}
}
