/*
 * Ugly, but I couldn't find a way to solve this witout the temporary variable,
 * when doing this iteratively.
 * 2014-04-13, Rust nightly (0.11-pre)
 */

fn main() {
	let mut sum : u32 = 0;
	let mut n : u32 = 1;
	let mut prev : u32 = 1;
	let mut tmp : u32;

	loop {
		tmp = n;
		n = n + prev;
		prev = tmp;

		if n % 2 == 0 { sum += n; }
		if n > 4000000 { break; }
	}

	println!("Answer: {}", sum);
}
