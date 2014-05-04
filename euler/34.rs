extern crate exscape;
use exscape::digits;
use std::iter::{range_inclusive, AdditiveIterator};

fn factorial(n: uint) -> uint {
	/* max input for 32-bit is 12, max for 64-bit is 20 */
	if n < 2 { return 1; }
	range_inclusive(2, n).fold(1, |a,b| a*b)
}

fn main() {
	let mut sum = 0;

	/*
	 * The lower limit is the smallest number that has a 2+ digit factorial.
	 * The upper limit is similar to that of problem 30.
	 * 9! = 362880 is the maximum result for one digit.
	 * For a 1-digit number, clearly the maximum result is way larger than the number.
	 * The same holds for 2-6 digit numbers as well (9! * 6 = 2 177 280, 7 digits).
	 * 9! * 7 is 7 digits, and 9! * 8 is also 7 digits, so for larger numbers,
	 * the sum of the factorials of the digits  will never reach the number itself.
	 */

	for n in range_inclusive(4u, 7 * factorial(9)) {
		if n == digits(n).iter().map(|&n| factorial(n)).sum() {
			sum += n;
		}
	}

	println!("Answer: {}", sum);
}
