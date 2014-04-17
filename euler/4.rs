/* 
 * 2014-04-15, Rust 0.11-pre.
 * First try was to iterate in reverse, but I didn't get that to work (using range*).
 * Doesn't quite matter unless I know when to break the loop, though; the first hit is
 * not necessarily the biggest product.
 */

extern crate exscape;
use exscape::is_palindrome_num;

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
