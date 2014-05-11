extern crate exscape;
use exscape::{is_triangular, is_pentagonal};

fn main() {
	// Since hexagonal numbers grow the fastest, I start by generating them, and testing
	// them for the other, more common properties. Start at H_144 since we know H_143
	// matches, from the problem statement.

	for n in std::iter::count(144u, 1) {
		let hex = n*(2*n - 1);
		if is_triangular(hex) && is_pentagonal(hex) {
			println!("Answer: {}", hex);
			break;
		}
	}
}
