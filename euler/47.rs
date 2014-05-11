extern crate exscape;

fn num_distinct_factors(n: uint) -> uint {
	let mut factors = exscape::factor(n);
	factors.dedup(); /* factors are already sorted */
	factors.len()
}

fn main() {
	static NUM : uint = 4; /* 2 and 3 for the examples, 4 for the problem */
	'outer: for i in std::iter::count(2u, 1) {
		for j in range(0, NUM) {
			if num_distinct_factors(i + j) != NUM {
				continue 'outer;
			}
		}

		println!("Answer: {}", i);
		break;
	}
}
