/* 2014-04-19, Rust 0.11-pre-nightly */

extern crate exscape;
extern crate collections;
use exscape::divisor_sum;
use collections::hashmap::HashSet;
use std::iter::AdditiveIterator;

#[inline(always)]
fn is_abundant(n: uint) -> bool {
	divisor_sum(n) > n
}

fn main() {
	/* All numbers in the problem range */
	let mut nums : HashSet<uint> = range(1u,28123).collect::<HashSet<uint>>();

	/* All abundant numbers in the problem range */
	let abundant = range(1u,28123).filter(|&n| is_abundant(n)).collect::<Vec<uint>>();

	/* Remove all possible sums of abundant numbers from nums */
	for i in abundant.iter() {
		for j in abundant.iter() {
			nums.remove(&(*i + *j));
		}
	}

	/* Only numbers that are not possible sums of abundant numbers remain; add them up, and we're done! */
	println!("Answer: {}", nums.iter().fold(0, |a,&b| a+b));
}
