extern crate exscape;
use exscape::is_prime;

fn main() {
	let mut max = (0i, 0i, 0i); /* (a, b, n) */

	for a in range(-999, 1000) {
		for b in range(2, 1000) { /* when n=0 and b < 2, the result will never be prime */
			let mut n = 0;

			loop {
				let p = n*n + a*n + b;
				if p < 2 || !is_prime(p as uint) { break; }
				n += 1;

				max = match max {
					(_, _, maxcount) if n > maxcount => (a, b, n),
					_ => max
				};
			}
		}
	}

	match max {
		(a, b, count) => println!("a={}, b={} (a*b={}) gave {} consecutive primes", a, b, a*b, count)
	}
}
