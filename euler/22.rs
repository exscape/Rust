/* 2014-04-19, Rust 0.11-pre-nightly */

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::iter::AdditiveIterator;

fn score(name: &str, pos: uint) -> uint {
	/* pos is 0-indexed, so we need to add 1; similarly, 'A' - 'A' is 0, so we need to add 1 there, too */
	(pos + 1) * name.chars().map(|c| (c as uint) - ('A' as uint) + 1).sum()
}

fn main() {
	let reader = File::open(&Path::new("22.txt")).unwrap();
	let mut reader = BufferedReader::new(reader);
	let s : ~str = reader.read_to_str().unwrap();

	let mut names = s.split(',').map(|s| s.trim_chars(&'"')).collect::<Vec<&str>>();
	names.sort();

	let mut sum = 0;

	for i in range(0, names.len()) {
		sum += score(*names.get(i), i);
	}

	println!("Answer: {}", sum);
}
