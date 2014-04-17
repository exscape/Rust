#![crate_id = "exscape#0.01"]

extern crate std;

pub fn factor(mut n: uint) -> Vec<uint> {
	let mut factors : Vec<uint> = Vec::new();

	let mut d : uint = 3;

	while n > 1 {
		while n % 2 == 0 {
			// Special case for the only even prime, so that we can
			// start at 3 and do += 2 below, instead of testing
			// 2, 3, 4, 5, 6, 7, 8 ... for no reason whatsoever.
			factors.push(2);
			n /= d;
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
