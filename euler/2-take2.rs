/*
 * 2014-04-17, Rust nightly (0.11-pre)
 */

extern crate exscape;
use exscape::Fibonacci;
use std::iter::AdditiveIterator;

fn main() {
	let fib = Fibonacci::new();
	println!("Answer: {}", fib.take_while(|&n| n < 4000000).filter(|&n| n % 2 == 0).sum());
}
