/*
 * Not the simplest way to solve this, but I wanted to implement
 * the prime sieve (Sieve of Eratosthenes) anyway.
 * Rust 0.11-pre, 20140413
 * ... it turns out, this problem can't be solved this way; the sieve
 * is way, way, way too slow for big numbers. The new algorithm (3-take2) is
 * takes 0.04% the time to factor a number 450 times as big!
 */

fn primes_up_to(n: uint) -> ~[uint] {

	fn sqrt(n: uint) -> uint {
		(n as f64).sqrt() as uint
	}

	let mut sieve = std::slice::from_elem(n, true);

	for i in range(2, sqrt(n)) {
		if sieve[i] {
			let mut j = i*i;
			while j < n {
				sieve[j] = false;
				j += i;
			}
		}
	}

	let mut primes: ~[uint] = ~[];
	for i in range(2, n) {
		if sieve[i] { primes.push(i); }
	}

	return primes;
}

fn factor(mut n: uint) -> ~[uint] {
	if n < 2 { fail!("cannot prime factorize number smaller than 2"); }

	let mut factors : ~[uint] = ~[];

	for p in primes_up_to(n/2 + 1).iter() { /* TODO: is +1 needed? */
		loop {
			if n % *p == 0 {
				factors.push(*p);
				n /= *p;
			}
			else {
				break;
			}
		}
	}

	if factors.len() == 0 {
		// Number is prime; we only tested up to n/2 though, so it won't be in the vector
		return ~[n];
	}
	else {
		return factors;
	}
}

fn main() {
	println!("{:?}", factor(1314952334));
//	println!("{:?}", factor(600851475143u));
}
