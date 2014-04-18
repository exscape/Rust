/* 2014-04-18, Rust 0.11-pre-nightly */

extern crate num;
use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use num::bigint::ToBigUint;

fn main() {
	let reader = File::open(&Path::new("13.txt")).unwrap();
	let mut reader = BufferedReader::new(reader);

	let mut sum = 0u.to_biguint().unwrap();

	for line in reader.lines() {
		let line = line.unwrap();
		sum = sum + from_str(line.trim()).unwrap();
	}

	println!("Answer: {}", sum.to_str().slice_to(10));
}
