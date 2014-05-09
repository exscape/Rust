/* 2014-04-19, Rust 0.11-pre-nightly */

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::iter::AdditiveIterator;

fn main() {
	let reader = File::open(&Path::new("22.txt")).unwrap();
	let mut reader = BufferedReader::new(reader);
	let s : ~str = reader.read_to_str().unwrap();

	let mut names = s.split(',').map(|s| s.trim_chars('"')).collect::<Vec<&str>>();
	names.sort();

	/* pos is 0-indexed, so we need to add 1; similarly, 'A' - 'A' is 0, so we need to add 1 there, too */
	let sum = names.iter().enumerate().map(|(pos, n)| {
	          n.chars().map(|c| (c as uint) - ('A' as uint) + 1).sum() * (pos + 1) /* this sum is letter values */
			  })
			  .sum(); /* and this sum is for the full name scores to the final sum */

	println!("Answer: {}", sum);
}
