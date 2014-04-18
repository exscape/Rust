#![crate_id = "exscape#0.01"]

extern crate std;
extern crate num;
use num::bigint::{ToBigUint, BigUint};

pub fn factor(mut n: uint) -> Vec<uint> {
/* Prime factorization of a number; returns a sorted Vec e.g. [2, 2, 2, 3, 5] for n = 120 */
	let mut factors : Vec<uint> = Vec::new();

	let mut d : uint = 3;

	while n > 1 {
		while n % 2 == 0 {
			// Special case for the only even prime, so that we can
			// start at 3 and do += 2 below, instead of testing
			// 2, 3, 4, 5, 6, 7, 8 ... for no reason whatsoever.
			factors.push(2);
			n /= 2;
		}

		while n % d == 0 {
			factors.push(d);
			n /= d;
		}
		d += 2;

		if d*d > n { /* or: if d > sqrt(n) */
			if n > 1 {
				factors.push(n);
			}
			break;
		}
	}
	
	return factors;
}

pub fn primes_up_to(n: uint) -> Vec<uint> {
	/* Sieve of Eratosthenes */
	if n < 2 { return Vec::new(); }
	let mut pvec = Vec::from_elem(n, true);

	/* If we do 2 as a special case, and then loop 3, 5, 7... by d += 2,
	 * the program is not measurably faster even for n = 20 million!
	 * Since it adds code complexity, I removed this optimization.
	 * I must admit, I would expect it to matter a whole lot. I suppose
	 * the time is spent elsewhere...
	 */

	let mut d = 2;
	while d*d < n { /* while d < sqrt(n) */
		if *pvec.get(d) {
			let mut j = d*d;
			while j < n {
				*pvec.get_mut(d) = false;
				j += d;
			}
		}
		d += 1;
	}

	let mut primes = Vec::new();
	for i in range(2, n) {
		if *pvec.get(i) {
			primes.push(i);
		}
	}

	primes
}

pub struct Fibonacci { prev: uint, cur: uint }

impl Fibonacci {
	pub fn new() -> Fibonacci {
		Fibonacci { prev: 0, cur: 1 }
	}
}

impl Iterator<uint> for Fibonacci {
	fn next(&mut self) -> Option<uint> {
		let tmp = self.cur;
		self.cur += self.prev;
		self.prev = tmp;
		Some(self.cur)
	}
}

pub fn is_palindrome(s: &str) -> bool {
	let filtered : ~str = s.chars().filter_map(|c| {
			match c.is_alphanumeric() {
				true => Some(c.to_lowercase()),
				false => None
			}
		}).collect();

	filtered == filtered.chars_rev().collect()
}

pub fn is_palindrome_num(n: uint) -> bool {
	is_palindrome(n.to_str())
}

pub fn isprime(n: uint) -> bool {
	if n > 2 && n % 2 == 0 { return false; }
	else if n == 2 { return true; }

	let mut d = 3u;

	while d*d <= n { /* while d <= sqrt(n) */
		if n % d == 0 {
			return false;
		}
		d += 2;
	}

	true
}

/* Returns the number of divisors of n, including 1 and the number itself. */
pub fn num_divisors(n: uint) -> uint {
	if n == 0 || n == 1 { return n; }
	let pr : Vec<uint> = factor(n); /* prime factorization; sorted "automatically" by the algorithm */
	if pr.len() == 1 { return 2; /* the prime factor itself plus being divisible by 1 */ }

	/*
	 * Algorithm: for prime factors x^a * y^b * z^c, there are (a+1)(b+1)(c+1) total divisors.
	 * In other words, we need to find the number of times each prime factor is repeated, add one to each, and multiply those together.
	 * 1080 = 2^3 * 3^3 * 5 would be represented as [2, 2, 2, 3, 3, 3, 5] here, and the result we want is then [3, 3, 1] -> 4 * 4 * 2 -> 32 divisors.
	 */

	let mut rep : Vec<uint> = Vec::new();

	let mut prev = 0;
	let mut count = 0;

	/* Count and push the number of repetitions (i.e. the exponents; 2*2*2*2 = 2^4 would push 4) */
	for &fact in pr.iter() {
		if prev == fact || prev == 0 { count += 1; }
		else { rep.push(count); count = 1; }
		prev = fact;
	}
	rep.push(count);

	/* rep now stores [a, b, c], so we add one to each, and multiply them together, to find the answer. */
	rep.iter().map(|x| x+1).fold(1, |a,b| a * b)
}

pub fn fac(n: uint) -> BigUint {
	let one = BigUint::new(vec!(1));
	if n == 0 { return BigUint::new(Vec::new()); }

	let mut num = n.to_biguint().unwrap();
	let mut result = 1u.to_biguint().unwrap();

	while num > one  {
		result = result * num;
		num = num - one;
	}

	result
}
