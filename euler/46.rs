extern crate exscape;
use exscape::{primes_up_to, is_prime};
use std::iter::{count, range_inclusive};

fn main() {
	let mut odd_composites = count(9u, 2).filter(|&n| !is_prime(n));

	'outer: for n in odd_composites {
		// For each candidate number n, try to construct the number.
		// If we succeed it's not the answer, so we skip to the next candidate.
		// If we fail ('primeloop exits normally), we have found the answer.
		// Optimizing the loop doesn't appear to save any time; startup time
		// probably dominates. Even eliminating 30% of iterations saves less than 1 ms.

		'primeloop: for p in primes_up_to(n).iter() {
			for s in range_inclusive(1u, (((n/2) as f64).sqrt() + 0.5) as uint) {
				if p + 2*s*s == n { continue 'outer; }
			}
		}

		println!("Answer: {}", n);
		break;
	}
}
