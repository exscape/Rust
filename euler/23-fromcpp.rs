/*
 * Credit to kbwt at the projecteuler.net forum for the solution.
 * I merely translated it to Rust (0.11-pre, 2014-04-19).
 */

use std::iter::range_step;

fn main() {
	let N : uint = 28123;

	let mut d = [0u, ..28123];
	let mut abundant = Vec::new();

	for i in range(1u, N) {
		for j in range_step(2*i, N, i) {
			d[j] += i;
		}

		if d[i] > i {
			abundant.push(i);
		}
	}

	let mut abundant_sum = [false, ..28123];
	for i in range(0u, abundant.len()) {
		for j in range(i, abundant.len()) {
			let sum = *abundant.get(i) + *abundant.get(j);
			if sum < N {
				abundant_sum[sum] = true;
			}
		}
	}

	let mut sum = 0;
	for i in range(0u, N) {
		if !abundant_sum[i] {
			sum += i;
		}
	}

	println!("Answer: {}", sum);
}
