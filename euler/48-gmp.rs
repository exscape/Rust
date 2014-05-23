extern crate gmp;
use std::iter::range_inclusive;
use gmp::Mpz;

fn main() {
	static LIMIT : uint = 1000;
	let mut sum = Mpz::new();

	for i in range_inclusive(1u, LIMIT) {
		sum = sum + std::num::pow(std::num::from_uint(i).unwrap(), i);
	}

	let sum_str = sum.to_str();
	println!("{}", sum_str.as_slice().slice_from(sum_str.len() - 10));
}
