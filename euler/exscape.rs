#![crate_id = "exscape#0.01"]

//! Various math functions.

extern crate std;
extern crate num;
use num::bigint::{ToBigUint, BigUint};
use std::num::pow;

/// Prime factorize `n` and return the result as a `Vec<uint>` of (possibly repeated) factors.
///
/// # Example
///
/// ```rust
/// use exscape::factor;
/// let factors = factor(120);
/// assert_eq!(factors, vec!(2, 2, 2, 3, 5));
/// ```
pub fn factor(mut n: uint) -> Vec<uint> {
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
	
	factors
}

/// Sieve of Eratosthenes. Returns a `Vec<uint>` of primes, up to and including `n`.
///
/// # Example
///
/// ```rust
/// use exscape::primes_up_to;
/// let primes = primes_up_to(11);
/// assert_eq!(primes, vec!(2, 3, 5, 7, 11));
/// ```
pub fn primes_up_to(n_in: uint) -> Vec<uint> {
	/* Sieve of Eratosthenes */
	let n = n_in + 1; /* better than using n+1 everywhere, at least; we need this to include `n` if `n` is prime. */
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
				*pvec.get_mut(j) = false;
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

/// A Fibonacci number iterator/generator.
///
/// # Example
///
/// ```rust
/// use exscape::Fibonacci;
/// let fib = Fibonacci::new();
/// let fibs : Vec<uint> = fib.take_while(|&c| c < 25).collect();
/// assert_eq!(fibs, vec!(1, 2, 3, 5, 8, 13, 21));
/// ```
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

/// Tests whether a string is a palindrome, testing only alphanumerics (ignoring case).
///
/// # Example
///
/// ```rust
/// use exscape::is_palindrome;
/// assert_eq!(is_palindrome(&"Go hang a salami, I'm a lasagna hog!"), true);
/// assert_eq!(is_palindrome(&"Hello, world!"), false);
/// ```
pub fn is_palindrome(s: &str) -> bool {
	let filtered : ~str = s.chars().filter_map(|c| {
			match c.is_alphanumeric() {
				true => Some(c.to_lowercase()),
				false => None
			}
		}).collect();

	filtered == filtered.chars_rev().collect()
}

/// Tests whether a uint is a palindrome.
///
/// # Example
///
/// ```rust
/// use exscape::is_palindrome_num;
/// let nums : Vec<uint> = vec!(0, 1, 52925, 34, 5062);
/// let results = nums.iter().map(|&n| is_palindrome_num(n)).collect::<Vec<bool>>();
/// assert_eq!(results, vec!(true, true, true, false, false));
/// ```
pub fn is_palindrome_num(n: uint) -> bool {
	is_palindrome(n.to_str())
}

/// Tests whether `n` is prime.
///
/// # Example
/// ```rust
/// use exscape::is_prime;
/// let nums = vec!(1, 2, 3, 4, 5);
/// let results = nums.iter().map(|&n| is_prime(n)).collect::<Vec<bool>>();
/// assert_eq!(results, vec!(false, true, true, false, true));
/// ```
pub fn is_prime(n: uint) -> bool {
	if n == 2 { return true; }
	else if n < 2 || (n > 2 && n % 2 == 0) { return false; }

	let mut d = 3u;

	while d*d <= n { /* while d <= sqrt(n) */
		if n % d == 0 {
			return false;
		}
		d += 2;
	}

	true
}

/// Returns the total number of divisors of `n`, including the number itself.
///
/// For the number of proper divisors, do `num_divisors(n) - 1`.
///
/// # Example
/// ```rust
/// use exscape::num_divisors;
/// let nums : Vec<uint> = vec!(1, 2, 6, 10);
/// let results = nums.iter().map(|&n| num_divisors(n)).collect::<Vec<uint>>();
/// assert_eq!(results, vec!(1, 2, 4, 4));
/// ```
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

/// Returns the factorial of `n`, `n!`, as a `BigUint`.
///
/// # Example
/// ```rust
/// use exscape::fac;
/// assert_eq!(fac(5).to_u64().unwrap(), 5*4*3*2*1);
/// ```
pub fn fac(n: uint) -> BigUint {
	let one = BigUint::new(vec!(1));
	if n == 0 || n == 1 { return one; } /* 0! == 1 by convention */

	let mut num = n.to_biguint().unwrap();
	let mut result = 1u.to_biguint().unwrap();

	while num > one  {
		result = result * num;
		num = num - one;
	}

	result
}

/* Returns the sum of all proper divisors of a number; that is, all divisors except the number itself; e.g. f(12) = (1+2+3+4+6) = 16 */
/// Returns the sum of all proper divisors of a number.
///
/// The proper divisors are all divisors except the number itself.
///
/// # Example
///
/// ```rust
/// use exscape::divisor_sum;
/// let div_sum = divisor_sum(12);
/// assert_eq!(div_sum, 1+2+3+4+6);
/// ```
pub fn divisor_sum(n: uint) -> uint {
	if n == 0 || n == 1 { return n; }

	let pr : Vec<uint> = factor(n); /* prime factorization; sorted "automatically" by the algorithm */
	if pr.len() == 1 { return 2; /* the prime factor itself plus being divisible by 1 */ }

	/*
	 * Algorithm: for prime factors x^a * y^b * z^c, there are (a+1)(b+1)(c+1) total divisors.
	 * In other words, we need to find the number of times each prime factor is repeated, add one to each, and multiply those together.
	 * 1080 = 2^3 * 3^3 * 5 would be represented as [2, 2, 2, 3, 3, 3, 5] here, and the result we want is then [3, 3, 1] -> 4 * 4 * 2 -> 32 divisors.
	 */

	let mut rep : Vec<Exp> = Vec::new();

	struct Exp { base: uint, exp: uint }

	let mut prev = 0;
	let mut count = 0;

	// Convert from [2, 2, 2, 3, 3, 5] style vectors to [2^3, 3^2, 5^1] style, using the above Exp struct
	for &fact in pr.iter() {
		if prev == fact || prev == 0 { count += 1; }
		else { rep.push(Exp{base: prev, exp: count}); count = 1; }
		prev = fact;
	}
	rep.push(Exp{base: prev, exp: count});

	/*
	 * Now we just need to calculate: divisor_sum = product of all (base^(exp+1) - 1)/(base-1),
	 * and finally subtract n (as the above formula yields the sum of ALL divisors, the number itself included).
	 */
	rep.iter().map(|&x| (pow(x.base, x.exp + 1) - 1)/(x.base - 1)).fold(1, |a,b| a*b) - n
}

#[test]
fn test_factor() {
	for i in std::iter::range_inclusive(0u, 3) {
		assert_eq!(factor(i), vec!(i));
	}

	assert_eq!(factor(4), vec!(2, 2));
	assert_eq!(factor(5), vec!(5));
	assert_eq!(factor(120), vec!(2, 2, 2, 3, 5));
	assert_eq!(factor(121), vec!(11, 11));
	assert_eq!(factor(127), vec!(127));
}

#[test]
fn test_primes_up_to() {
	assert_eq!(primes_up_to(0), Vec::new());
	assert_eq!(primes_up_to(1), Vec::new());
	assert_eq!(primes_up_to(2), vec!(2));
	assert_eq!(primes_up_to(3), vec!(2, 3));
	assert_eq!(primes_up_to(4), vec!(2, 3));
	assert_eq!(primes_up_to(13), vec!(2, 3, 5, 7, 11, 13));
	assert_eq!(primes_up_to(23), vec!(2, 3, 5, 7, 11, 13, 17, 19, 23));
}

#[test]
fn test_fibonacci() {
	let mut fib = Fibonacci::new();
	assert!(fib.next().unwrap() == 1);
	assert!(fib.next().unwrap() == 1);
	assert!(fib.next().unwrap() == 2);
	assert!(fib.next().unwrap() == 3);
	assert!(fib.next().unwrap() == 5);
	assert!(fib.next().unwrap() == 8);
}

#[test]
fn test_is_palindrome() {
	assert!(is_palindrome("Was it a car or a cat I saw?"));
	assert!(is_palindrome("Rats live on no evil star"));
	assert!(is_palindrome(r#"No "x" in "Nixon"#));
	assert!(is_palindrome("123 21"));
	assert!(!is_palindrome("123"));
	assert!(!is_palindrome("Rust is awesome"));
	assert!(!is_palindrome("Not a palindrome"));
}

#[test]
fn test_is_palindrome_num() {
	assert!(is_palindrome_num(0));
	assert!(is_palindrome_num(1));
	assert!(is_palindrome_num(101));
	assert!(is_palindrome_num(492858294));
	assert!(is_palindrome_num(9009));
	assert!(!is_palindrome_num(10));
	assert!(!is_palindrome_num(3523));
	assert!(!is_palindrome_num(10010));
	assert!(!is_palindrome_num(2940));
}

#[test]
fn test_is_prime() {
	assert!(!is_prime(0));
	assert!(!is_prime(1));
	assert!(!is_prime(4));
	assert!(!is_prime(6));
	assert!(!is_prime(121));
	assert!(!is_prime(1299829));
	assert!(is_prime(2));
	assert!(is_prime(3));
	assert!(is_prime(5));
	assert!(is_prime(127));
	assert!(is_prime(1299827));
}

#[test]
fn test_num_divisors() {
	assert_eq!(num_divisors(0), 0);
	assert_eq!(num_divisors(1), 1);
	assert_eq!(num_divisors(2), 2);
	assert_eq!(num_divisors(3), 2);
	assert_eq!(num_divisors(4), 3);
	assert_eq!(num_divisors(12), 6);
	assert_eq!(num_divisors(127), 2);
}

#[test]
fn test_fac() {
	assert_eq!(fac(0).to_u32().unwrap(), 1);
	assert_eq!(fac(1).to_u32().unwrap(), 1);
	assert_eq!(fac(2).to_u32().unwrap(), 2);
	assert_eq!(fac(3).to_u32().unwrap(), 6);
	assert_eq!(fac(10).to_u32().unwrap(), 3628800);
	assert_eq!(fac(12).to_u32().unwrap(), 479001600);
	assert_eq!(fac(15).to_u64().unwrap(), 1307674368000);
}

#[test]
fn test_divisor_sum() {
	assert_eq!(divisor_sum(0), 0);
	assert_eq!(divisor_sum(1), 1);
	assert_eq!(divisor_sum(2), 1);
	assert_eq!(divisor_sum(3), 1);
	assert_eq!(divisor_sum(4), 3);
	assert_eq!(divisor_sum(6), 6);
	assert_eq!(divisor_sum(12), 1+2+3+4+6);
	assert_eq!(divisor_sum(127), 1);
}
