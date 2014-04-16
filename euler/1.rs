/* Rust 0.11-pre, 20140413 */
fn main() {
	let max = 1000;

	let mut sum = 0;
	for num in range(1, max) {
		if num % 3 == 0 || num % 5 == 0 {
			sum += num;
		}
	}

	println!("Sum is {}", sum);
}
