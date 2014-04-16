use std::iter::range_inclusive;
use std::iter::AdditiveIterator; // .fold(0, |a, b| a + b) works instead of .sum() also

fn main() {
	let max = 100;

	let sum_squares = range_inclusive(1, max).map(|x| x * x).sum();
	let mut square_sums = range_inclusive(1, max).sum();
	square_sums *= square_sums; /* Ugh! Is there a better way, that doesn't require mut? */

	println!("sum_squares = {}", sum_squares);
	println!("square_sums = {}", square_sums);
	println!("Difference: {}", square_sums - sum_squares);

}
