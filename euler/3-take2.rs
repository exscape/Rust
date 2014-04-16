fn factor(mut n: uint) -> ~[uint] {
	let mut factors = ~[];

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

fn main() {
	let num = 18446744073709551613;
	let factors = factor(num);
	println!("{}: {:?}", num, factors);
	println!("Biggest factor: {}", factors.last().unwrap());
}
