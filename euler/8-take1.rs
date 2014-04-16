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
		let line = line.trim();
		buf.push_str(line);
	}
	let s = buf.into_owned();

	/* OK, so we can finally get to work. */
	let mut max : uint = 0;
	for i in range_inclusive(0u, 1000u - 5u) {
		// Start at position i, multiply s[i+0] * s[i+1] ... * s[i+4]
		let mut n : uint = 1;
		for j in range(0u, 5u) {
			n *= (s[i+j] - '0' as u8) as uint; /* TODO: use something more idiomatic than ASCII subtraction */
		}
		if n > max { max = n }
	}

	println!("Max = {}", max);
}
