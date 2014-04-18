extern crate exscape;
use d = exscape::divisor_sum;

fn main() {
	let mut sum = 0;
	for a in range(1u, 10000) {
		let b = d(a);
		if a != b && d(b) == a {
			sum += a; /* If we add both, we end up with exactly twice the answer, as we will first first e.g. 220, then 284,
					   * and therefore add each number twice. */
		}
	}
	println!("Answer: {}", sum);
}
