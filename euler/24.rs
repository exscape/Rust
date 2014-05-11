use std::iter::AdditiveIterator;

fn main() {
	let v = &mut[0u, 1, 2, 3, 4, 5, 6, 7, 8, 9];

	for _ in range(0, 999_999) {
		 v.next_permutation();
	}

	let n = v.iter().enumerate().map(|(pos,val)| val * std::num::pow(10u, v.len() - pos - 1)).sum();
	println!("Answer: {}", n);
}
