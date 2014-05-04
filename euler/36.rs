extern crate exscape;
use exscape::is_palindrome;
use std::num::ToStrRadix;
use std::iter::AdditiveIterator;

fn main() {
	let sum = range(1, 1_000_000).filter(|n| is_palindrome(n.to_str_radix(2)) && is_palindrome(n.to_str_radix(10))).sum();
	println!("Answer: {}", sum);
}
