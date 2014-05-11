use std::iter::range_step;

fn is_pentagonal(num: uint) -> bool {
	// If (sqrt(24x+1)+1) / 6 is an integer, x is pentagonal.
	let f = num as f64;
	let n = ((24.0 * f + 1.0).sqrt() + 1.0)/6.0;
	 n == n.trunc()
}

fn main() {
	// Strategy: Generate increasing pentagonals numbers P_n in an outer loop,
	// and decreasing (P_(n-1) and downwards) in an inner loop; test sums
	// and differences for "pentagonality".
	// When P_n - P_(n-1) > the smallest found answer, we can stop trying,
	// as larger numbers will never yield a smaller difference.

	let mut smallest = std::int::MAX;

	'outer: for i in std::iter::count(1, 1) {
		for j in range_step(i - 1, 0, -1) {
			let p2 = i*(3*i - 1)/2;
			let p1 = j*(3*j - 1)/2;

			if p2 - p1 > smallest { break 'outer; }

			if is_pentagonal((p2 - p1) as uint) && is_pentagonal((p2 + p1) as uint) {
				println!("{} and {}", p2, p1);
				if p2 - p1 < smallest { smallest = p2-p1; }
			}
		}
	}

	println!("Answer: {}", smallest);
}
