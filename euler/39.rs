use std::iter::range_inclusive;

fn main() {
	let mut max_solutions = 0;
	let mut max_p = 0;

	for p in range_inclusive(1u, 1000) {
		let mut solutions = 0;

		for a in range(1u, p/2) {
			for b in range(a, (p - a)/2) {
				// b > a was chosen to avoid duplicate solutions {a, b, c} and {b, a, c}.
				// Upper limit is a bit arbitrary, but if we let b = p - a, then b = (a+b+c) - a = b+c, so c = 0.
				// Clearly, that can never be a valid solution; c > b is guaranteed.
				let c = p - a - b;
				if c*c == a*a + b*b {
					solutions += 1;
				}
			}
		}

		if solutions > max_solutions {
			max_solutions = solutions;
			max_p = p;
		}
	}

	println!("Max = {} with {} solutions", max_p, max_solutions);
}
