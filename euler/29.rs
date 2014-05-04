extern crate num;
use num::bigint::{BigUint, ToBigUint};
use std::iter::range_inclusive;

fn main() {
	// BigUint doesn't support hashing, so we can't use HashSet.
	let mut v : Vec<BigUint> = Vec::new();

	for a in range_inclusive(2u, 100u) {
		for b in range_inclusive(2u, 100) {
			v.push(std::num::pow(a.to_biguint().unwrap(), b));
		}
	}

	v.sort();
	v.dedup();

	println!("Answer: {}", v.len());
}
