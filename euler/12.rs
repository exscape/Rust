extern crate exscape;
use exscape::num_divisors;
use std::iter::count;

fn tri(n: uint) -> uint {
	n*(n + 1)/2 /* range_inclusive(1, n).sum() == n * (n+1))/2 */
}

fn main() {
	for i in count(1u,1u) {
		let d = num_divisors(tri(i));
		if d > 500 { println!("{}: {} divisors", tri(i), d); break; }
	}
}
