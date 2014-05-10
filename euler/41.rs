extern crate exscape;
use exscape::{primes_up_to, is_pandigital};

fn main() {
	// 8-digit and 9-digit pandigitals will all be divisible by 3,
	// as their digital sums are divisible by 3. Therefore, none of them are prime.
	for &p in primes_up_to(7654321).iter().rev() { 
		if is_pandigital(p) {
			println!("Answer: {}", p);
			break;
		}
	}
}
