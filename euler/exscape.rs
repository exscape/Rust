#![crate_id = "exscape#0.01"]

//! Various math functions.

extern crate std;
extern crate num;
extern crate gmp;
extern crate test;
use num::bigint::{ToBigUint, BigUint};
use std::num::pow;
use std::iter::range_inclusive;
use gmp::Mpz;

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

	if n < 2 {
		/* Mathematica does it this way, so it should be OK enough. */
		factors.push(n);
		return factors;
	}

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
/// assert_eq!(fibs, vec!(1, 1, 2, 3, 5, 8, 13, 21));
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
		Some(self.prev)
	}
}

/// Tests whether a string is a palindrome, testing only alphanumerics (ignoring case).
///
/// # Example
///
/// ```rust
/// use exscape::is_palindrome;
/// assert_eq!(is_palindrome("Go hang a salami, I'm a lasagna hog!"), true);
/// assert_eq!(is_palindrome("Hello, world!"), false);
/// ```
pub fn is_palindrome(s: &str) -> bool {
	let filtered : StrBuf = s.chars().filter_map(|c| {
			match c.is_alphanumeric() {
				true => Some(c.to_lowercase()),
				false => None
			}
		}).collect();

	filtered == filtered.as_slice().chars().rev().collect()
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
#[inline]
pub fn is_palindrome_num(n: uint) -> bool {
	reverse_num(n) == n
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
	if pr.len() == 1 { return 1; /* since we exclude the number itself, the divisor sum is 1 for all primes */ }

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

/// Returns the digits of a number, in order.
///
/// # Example
///
/// ```rust
/// use exscape::digits;
/// let d = digits(12345);
/// assert_eq!(d, vec!(1, 2, 3, 4, 5));
/// ```
pub fn digits(mut n: uint) -> Vec<uint> {
	let mut out = Vec::new();

	loop {
		out.push(n % 10);
		n /= 10;
		if n == 0 { break; }
	}

	out.reverse();
	out
}

/// Returns the number of digits of `n` (log10(n) + 1, though 0 is a special case).
///
/// # Example
///
/// ```rust
/// use exscape::num_digits;
/// assert_eq!(num_digits(0), 1);
/// assert_eq!(num_digits(90400), 5);
/// ```
#[inline]
pub fn num_digits(n: uint) -> uint {
	if n < 10 { return 1; }
	if n > std::num::pow(2u,53) { fail!("log() out of valid range; answer wouldn't be exact"); }

	1 + (n as f64).log10() as uint
}

/// Rotates a number, moving the last digit to the first place.
///
/// Note that e.g. 10 rotates to 1, as the leading zero is ignored.
/// This also implies that rotating a n-digit number n times isn't guaranteed
/// to produce the original number.
///
/// # Example
///
/// ```rust
/// use exscape::rotate_num;
/// assert_eq!(rotate_num(123), 312);
/// assert_eq!(rotate_num(100), 10);
/// assert_eq!(rotate_num(101), 110);
/// ```
#[inline]
pub fn rotate_num(mut n: uint) -> uint {
	if n < 10 { return n; }

	// fetch last digit
	let last = n % 10;
	// remove it
	n /= 10;
	// add it back in front, and return
	n + last * std::num::pow(10u, num_digits(n))
}

/// Returns `true` if `num` contains each digit from 1 to `n` exactly once.
pub fn is_pandigital(num: uint) -> bool {
	let mut v = digits(num);
	v.sort();
	v == range(1, num_digits(num) + 1).collect::<Vec<uint>>()
}

/// Truncate a number from the right. 1-digit numbers truncate to 0.
///
/// # Example
///
/// ```rust
/// use exscape::trunc_right;
/// assert_eq!(trunc_right(123), 12);
/// assert_eq!(trunc_right(1001), 100);
/// assert_eq!(trunc_right(9), 0);
/// ```
#[inline]
pub fn trunc_right(n: uint) -> uint {
	n / 10
}

/// Truncate a number from the left. 1-digit numbers truncate to 0.
///
/// # Example
///
/// ```rust
/// use exscape::trunc_left;
/// assert_eq!(trunc_left(123), 23);
/// assert_eq!(trunc_left(1001), 1); // since 001 == 1
/// assert_eq!(trunc_left(9), 0);
/// ```
#[inline]
pub fn trunc_left(n: uint) -> uint {
	n % std::num::pow(10u, num_digits(n) - 1)
}

/// Returns whether `num` is triangular, i.e. whether it can be written as n*(n+1)/2 for integer `n`.
pub fn is_triangular(n: uint) -> bool {
	// If and only if 8n + 1 is a square, n is a triangular number.
	let root = ((8*n + 1) as f64).sqrt() as uint;
	root*root == 8*n + 1
}

/// Returns whether `num` is pentagonal, i.e. whether it can be written as n*(3n-1)/2 for integer `n`.
pub fn is_pentagonal(num: uint) -> bool {
	// If (sqrt(24x+1)+1) / 6 is an integer, x is pentagonal.
	let f = num as f64;
	let n = ((24.0 * f + 1.0).sqrt() + 1.0)/6.0;
	 n == n.trunc()
}

/// Returns whether `num` is hexagonal, i.e. whether it can be written as n*(2n-1) for integer `n`.
pub fn is_hexagonal(num: uint) -> bool {
	// If (sqrt(8x+1) + 1) / 4 is an integer, x is hexagonal.
	let f = num as f64;
	let n = ((8.0 * f + 1.0).sqrt() + 1.0)/4.0;
	 n == n.trunc()
}

/// Returns `true` if `n1` and `n2` are permutations of each other, i.e. made up of the same digits.
///
/// Leading zeroes are ignored, so while 1230 and 1203 are considered permutations of each other, 1230 and 0123 are not.
pub fn is_permutation(n1: uint, n2: uint) -> bool {
	let mut v1 = digits(n1);
	let mut v2 = digits(n2);
	v1.sort();
	v2.sort();

	v1 == v2
}

/// A factorial using GMP's Mpz type, whose size is limited only by memory usage.
///
/// For example, calculating fac_mpz(1000) is not a problem, despite it being over 2500 digits.
pub fn fac_mpz(n_in: uint) -> Mpz {
	if n_in <= 1 { return std::num::one(); }

	let mut prod : Mpz = std::num::one();
	for i in range_inclusive(2u64, n_in as u64) {
		prod = prod * FromPrimitive::from_u64(i).unwrap();
	}

	prod
}

/// Reverses a number. Note that leading zeroes are ignored, so 120 reversed is just 21.
pub fn reverse_num(mut n: uint) -> uint {
	// Using digits().iter().rev().collect() and converting back to uint would be easier,
	// but also almost certainly slower.

	let mut rev = 0;
	let len = num_digits(n);

	for i in range(0, len) {
		rev += (n % 10) * std::num::pow(10u, len - i - 1);
		n /= 10;
	}

	rev
}

#[test]
fn test_reverse_num() {
	assert_eq!(reverse_num(0), 0);
	assert_eq!(reverse_num(1), 1);
	assert_eq!(reverse_num(9), 9);
	assert_eq!(reverse_num(10), 01);
	assert_eq!(reverse_num(11), 11);
	assert_eq!(reverse_num(12), 21);
	assert_eq!(reverse_num(98), 89);
	assert_eq!(reverse_num(123), 321);
	assert_eq!(reverse_num(1230), 0321);
	assert_eq!(reverse_num(1203), 3021);
	assert_eq!(reverse_num(3458295), 5928543);
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
	assert_eq!(primes_up_to(12), vec!(2, 3, 5, 7, 11));
	assert_eq!(primes_up_to(13), vec!(2, 3, 5, 7, 11, 13));
	assert_eq!(primes_up_to(14), vec!(2, 3, 5, 7, 11, 13));
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

#[test]
fn test_digits() {
	assert_eq!(digits(0), vec!(0));
	assert_eq!(digits(1), vec!(1));
	assert_eq!(digits(10), vec!(1, 0));
	assert_eq!(digits(99), vec!(9, 9));
	assert_eq!(digits(194815), vec!(1, 9, 4, 8, 1, 5));
}

#[test]
fn test_num_digits() {
	assert_eq!(num_digits(0), 1);
	assert_eq!(num_digits(1), 1);
	assert_eq!(num_digits(9), 1);
	assert_eq!(num_digits(10), 2);
	assert_eq!(num_digits(11), 2);
	assert_eq!(num_digits(19), 2);
	assert_eq!(num_digits(20), 2);
	assert_eq!(num_digits(21), 2);
	assert_eq!(num_digits(99), 2);
	assert_eq!(num_digits(99), 2);
	assert_eq!(num_digits(100), 3);
	assert_eq!(num_digits(101), 3);
	assert_eq!(num_digits(104900), 6);
}

#[test]
fn test_rotate_num() {
	assert_eq!(rotate_num(0), 0);
	assert_eq!(rotate_num(1), 1);
	assert_eq!(rotate_num(9), 9);
	assert_eq!(rotate_num(10), 1);
	assert_eq!(rotate_num(100), 10);
	assert_eq!(rotate_num(101), 110);
	assert_eq!(rotate_num(197), 719);
	assert_eq!(rotate_num(719), 971);
}

#[test]
fn test_trunc_right() {
	assert_eq!(trunc_right(0), 0);
	assert_eq!(trunc_right(1), 0);
	assert_eq!(trunc_right(9), 0);
	assert_eq!(trunc_right(10), 1);
	assert_eq!(trunc_right(11), 1);
	assert_eq!(trunc_right(19), 1);
	assert_eq!(trunc_right(20), 2);
	assert_eq!(trunc_right(21), 2);
	assert_eq!(trunc_right(29), 2);
	assert_eq!(trunc_right(12000), 1200);
	assert_eq!(trunc_right(12001), 1200);
	assert_eq!(trunc_right(12345), 1234);
}

#[test]
fn test_trunc_left() {
	assert_eq!(trunc_left(0), 0);
	assert_eq!(trunc_left(1), 0);
	assert_eq!(trunc_left(9), 0);
	assert_eq!(trunc_left(10), 0);
	assert_eq!(trunc_left(11), 1);
	assert_eq!(trunc_left(19), 9);
	assert_eq!(trunc_left(20), 0);
	assert_eq!(trunc_left(21), 1);
	assert_eq!(trunc_left(29), 9);
	assert_eq!(trunc_left(12000), 2000);
	assert_eq!(trunc_left(12001), 2001);
	assert_eq!(trunc_left(12345), 2345);
}

#[test]
fn test_is_pandigital() {
	assert_eq!(is_pandigital(12345), true);
	assert_eq!(is_pandigital(12344), false);
	assert_eq!(is_pandigital(1), true);
	assert_eq!(is_pandigital(2), false);
	assert_eq!(is_pandigital(12), true);
	assert_eq!(is_pandigital(13), false);
	assert_eq!(is_pandigital(120), false);
	assert_eq!(is_pandigital(987654321), true);
	assert_eq!(is_pandigital(197246583), true);
	assert_eq!(is_pandigital(54316), false);
	assert_eq!(is_pandigital(54316), false);
}
