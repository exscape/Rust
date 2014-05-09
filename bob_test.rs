// Checks if each line in bob.txt (lyrics to Weird Al - Bob) is a palindrome.

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;

fn is_palindrome(s: &str) -> bool {
	let filtered : ~str = s.chars().filter_map(|c| {
			match c.is_alphanumeric() {
				true => Some(c.to_lowercase()),
				false => None
			}
		}).collect();

	filtered == filtered.chars().rev().collect()
}

fn main() {
	let path = Path::new("bob.txt");
	let reader = File::open(&path).unwrap();
	let mut reader = BufferedReader::new(reader);

	for line in reader.lines() {
		let line = line.unwrap();
		let line = line.trim();
		println!("{} for {}", is_palindrome(line), line);
	}
}
