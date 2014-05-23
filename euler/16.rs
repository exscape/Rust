/* 2014-04-18 (updated 2014-05-04), Rust 0.11-pre-nightly */

extern crate num;
use num::bigint::ToBigUint;
use std::iter::AdditiveIterator;

fn main() {
	let num = std::num::pow(2u.to_biguint().unwrap(), 1000);
	println!("Answer: {}", num.to_str().as_slice().chars().map(|c| c.to_digit(10).unwrap()).sum());
}
