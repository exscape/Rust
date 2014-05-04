extern crate exscape;
use exscape::digits;
use std::iter::{range_inclusive, AdditiveIterator};

fn main() {
	/*
	 * 9^5 = 59049 is the largest value a digit can have.
	 * We only need to look as long as 9^5*n is bigger than the largest possible
	 * number made up of n digits. So 9^5*2 > 99, 9^5*3 > 999, 9^5*4 > 9999 ...
	 * ... but 9^5*6 < 999999 and 9^5*7 < 9999999, and so on.
	 * Once we've tested to that limit, no larger number can ever match.
	 */

	let mut sum = 0;
	for n in range_inclusive(10u, 354294) { /* 9^5 * 6 = 354294 */
		if n == digits(n).iter().map(|&n| std::num::pow(n, 5)).sum() {
			sum += n;
		}
	}

	println!("Answer: {}", sum);
}
