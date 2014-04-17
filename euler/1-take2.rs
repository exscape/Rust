/* Rust 0.11-pre, 20140417 */
use std::iter::AdditiveIterator;
fn main() {
	println!("{}", range(1,1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum());
}
