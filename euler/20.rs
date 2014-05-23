/* 2014-04-18, Rust 0.11-pre-nightly */

extern crate exscape;
use std::iter::AdditiveIterator;
use exscape::fac;

fn main() {
	let num = fac(100);
	println!("Answer: {}", num.to_str().as_slice().chars().map(|c| c.to_digit(10).unwrap()).sum());
}
