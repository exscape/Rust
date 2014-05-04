extern crate exscape;
use exscape::{num_digits, rotate_num, primes_up_to, is_prime};

fn main() {
	let mut count = 0;

	// Using the sieve is only marginally faster than trying all
	// the numbers in range(2, 1_000_000) for primality... 
	// But it *is* faster.
	// Numbers containing 0, 2, 4, 6 or 8 will never be circular primes,
	// but it's easier and quite possibly faster to just test all numbers.
	for & mut n in primes_up_to(1_000_000).iter() {
		let mut hit = true;

		for _ in range(0, num_digits(n)) {
			if !is_prime(n) { hit = false; break; }
			n = rotate_num(n);
		}

		if hit { count += 1; }
	}

	println!("Answer: {}", count);
}
