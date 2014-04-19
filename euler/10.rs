extern crate exscape;
use exscape::primes_up_to;

fn main() {
	let max = 2000000;
	let primes = primes_up_to(max);

	let sum = primes.iter().fold(0, |a,&b| (a as u64) + (b as u64));
	println!("Sum: {}", sum);
}
