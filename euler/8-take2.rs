use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use std::strbuf::StrBuf;
use std::iter::range_inclusive;

fn main() {
	let reader = File::open(&Path::new("8.txt")).unwrap();
	let mut reader = BufferedReader::new(reader);
	let mut buf = StrBuf::new();

	for line in reader.lines() {
		let line = line.unwrap();
		buf.push_str(line.trim());
	}

	/* OK, so we can finally get to work. */
	let mut max : uint = 0;
	for i in range_inclusive(0u, 1000u - 5u) {
		// Start at position i, multiply n by s[i+0] * s[i+1] ... * s[i+4]
		let mut n : uint = 1;

		for c in buf.as_slice().slice(i, i+5).chars() {
			n *= c.to_digit(10).unwrap();
		}

		if n > max { max = n }
	}

	println!("Max = {}", max);
}
