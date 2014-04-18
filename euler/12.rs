extern crate exscape;
use exscape::factor;
use std::iter::count;

fn tri(n: uint) -> uint {
	n*(n + 1)/2 /* range_inclusive(1, n).sum() == n * (n+1))/2 */
}

fn num_divisors(n: uint) -> uint {
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

	/* Count the number of repetitions, and push the number (i.e. the exponents; 2*2*2*2 = 2^4 would push 4) */
	for &fact in pr.iter() {
		if prev == fact || prev == 0 { count += 1; }
		else { rep.push(count); count = 1; }
		prev = fact;
	}
	rep.push(count);

	/* rep now stores [a, b, c], so we add one to each, and multiply them together, to find the answer. */
	rep.iter().map(|x| x+1).fold(1, |a,b| a * b)
}

fn main() {
	for i in count(1u,1u) {
		let d = num_divisors(tri(i));
		if d > 500 { println!("{}: {} divisors", tri(i), d); break; }
	}
}
