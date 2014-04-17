extern crate exscape;
use exscape::factor;

fn tri(n: uint) -> uint {
	n*(n + 1)/2
}

fn num_divisors(n: uint) -> uint {
	let mut pr : Vec<uint> = factor(n); /* prime factors, e.g. [2, 2, 2, 3, 5] for tri(15) = 120 */
	pr.sort();
	let pr = pr;

	/* 
	 * Algorithm: for prime factors x^a * y^b * z^c, there are (a+1)(b+1)(c+1) total factors.
	 * In other words, we need to find the number of times each prime factor is repeated, add one to each, and multiply those together.
	 */

	let mut rep : Vec<uint> = Vec::new();

	let mut prev : uint = 0;
	let mut count : uint = 0;
	for &fac in pr.iter() {
		if fac == prev || prev == 0 {
			count += 1;
		}
		else if fac != prev {
			rep.push(count);
			count = 1;
		}
		prev = fac;
	}
	rep.push(count);

	rep.iter().map(|x| x+1).fold(1, |a,b| a * b)
}

fn main() {
	let mut i = 1;
	loop {
		let d = num_divisors(tri(i));
		if d > 500 { println!("{}: {} divisors", tri(i), d); break; }
		i += 1;
	}
}
