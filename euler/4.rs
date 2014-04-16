/* 
 * 2014-04-15, Rust 0.11-pre.
 * First try was to iterate in reverse, but I didn't get that to work (using range*).
 * Doesn't quite matter unless I know when to break the loop, though; the first hit is
 * not necessarily the biggest product.
 */

fn is_palindrome(s: &str) -> bool {
	let filtered : ~str = s.chars().filter_map(|c| {
			match c.is_alphanumeric() {
				true => Some(c.to_lowercase()),
				false => None
			}
		}).collect();

	filtered == filtered.chars_rev().collect()
}

fn is_palindrome_num(n: uint) -> bool {
	is_palindrome(n.to_str())
}

fn main() {
	// Yay, brute force!
	let mut max : uint = 0;
	for i in range(100u, 999u) {
		for j in range(i, 999u) {
			if is_palindrome_num(i*j) && i*j > max {
				max = i*j;
			}
		}
	}
	println!("Max = {}", max);
}
