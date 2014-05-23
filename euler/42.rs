extern crate exscape;
use exscape::is_triangular;
use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::iter::AdditiveIterator;

fn main() {
	let reader = File::open(&Path::new("42.txt")).unwrap();
	let mut reader = BufferedReader::new(reader);
	let s : StrBuf = reader.read_to_str().unwrap();

	let mut words = s.as_slice().split(',').map(|s| s.trim_chars('"')).collect::<Vec<&str>>();
	words.sort();

	/* pos is 0-indexed, so we need to add 1; similarly, 'A' - 'A' is 0, so we need to add 1 there, too */
	let num = words.iter().map(|n| {
	          n.as_slice().chars().map(|c| (c as uint) - ('A' as uint) + 1).sum()
			  }).filter(|&n| is_triangular(n)).len();

	println!("Answer: {}", num);
}
