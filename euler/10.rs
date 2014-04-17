fn primes_up_to(n: uint) -> ~[uint] {
	/* Sieve of Eratosthenes */
	if n < 2 { return ~[]; }
	let mut pvec = std::slice::from_elem(n, true);

	/* If we do 2 as a special case, and then loop 3, 5, 7... by d += 2,
	 * the program is not measurably faster even for n = 20 million!
	 * Since it adds code complexity, I removed this optimization.
	 * I must admit, I would expect it to matter a whole lot. I suppose
	 * the time is spent elsewhere...
	 */

	let mut d = 2;
	while d*d < n { /* while d < sqrt(n) */
		if pvec[d] {
			let mut j = d*d;
			while j < n {
				pvec[j] = false;
				j += d;
			}
		}
		d += 1;
	}

	let mut primes = ~[];
	for i in range(2, n) {
		if pvec[i] {
			primes.push(i);
		}
	}

	primes
}

fn main() {
	let max = 2000000;
	let primes = primes_up_to(max);

	let sum = primes.iter().fold(0, |a,&b| (a as u64) + (b as u64));
	println!("Sum: {}", sum);
}
