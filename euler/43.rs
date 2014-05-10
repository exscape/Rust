extern crate exscape;
use exscape::primes_up_to;
use std::iter::AdditiveIterator;

fn join_num(s: &[uint]) -> uint {
	s.iter().enumerate().map(|(pos,&val)| val * std::num::pow(10u, s.len() - pos - 1)).sum()
}

fn main() {
	// Start at a permutation without a leading zero, as all those won't be 0-9 pandigitals.
	// The first permutation is not tested, but that's OK, since 023 % 2 != 0, so it's not a match.
	let num = &mut[1u, 0, 2, 3, 4, 5, 6, 7, 8, 9];
	let primes = primes_up_to(17);
	let mut sum = 0;

	'outer: loop {
		if !num.next_permutation() {
			break;
		}

		for i in range(1u, 8) {
			let n = join_num(num.slice(i, i+3));
			if n % *primes.get(i - 1) != 0 { continue 'outer; }
		}

		sum += join_num(num);
	}

	println!("Answer: {}", sum);
}
