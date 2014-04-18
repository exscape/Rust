/* 2014-04-18, Rust 0.11-pre-nightly */

extern crate num;
use num::bigint::ToBigUint;
use std::iter::AdditiveIterator;

/*
 * Not very elegant, but it works. I can't find ways to improve it at the moment, either:
 * 1) 2u.to_biguint().unwrap() and from_u32::<BigUint>(2).unwrap() are the ways I've found to create BigUints
 * 2) There's no pow() operator or similar, so repeated multiplication appears to be required
 * 3) num *= two doesn't work,
 * 4) num = num * 2 doesn't work, either
 * 5) There's no iterator over the digits, so the to_str() will have to do.
 */

fn main() {
	let two = 2u.to_biguint().unwrap();
	let mut num = two.clone();

	for _ in range(1, 1000) { num = num * two }

	println!("Answer: {}", num.to_str().chars().map(|c| c.to_digit(10).unwrap()).sum());
}
