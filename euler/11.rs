/* 2014-04-16, Rust 0.11-pre-nightly */

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;

enum Direction { Horizontal, Vertical, DownRight, UpRight }
struct Range { min: uint, max: uint }

fn main() {
	let reader = File::open(&Path::new("11.txt")).unwrap();
	let mut reader = BufferedReader::new(reader);

	let mut v : Vec<Vec<int>> = Vec::new();

	for line in reader.lines() {
		let line = line.unwrap();
		/* Split into numbers, convert each part into an int using from_str(), collect them into a Vec<int> and push it. */
		/* from_str(s) works as well as from_str::<int>(s) but is less clear. */

		v.push(line.as_slice().trim().split(' ').map(|s| from_str::<int>(s).unwrap()).collect::<Vec<int>>());
	}

	/*
	 * Alright, let's get to work!
	 * check() is called with a x range, y range and direction to check.
	 * The first call starts it at (x, y) = (0, 0) and tests 4 to the right,
	 * the next at (1, 0) and tests 4 to the right, etc.
	 * Next, it does the same but tests 4 downwards, etc.
	 * Finally, it tests the diagonals by changing both x and y for each of the 4 inner iterations.
	 */
	let check = |x: Range, y: Range, dir: Direction, max: &mut int| {
		for x in range(x.min, x.max) {
			for y in range(y.min, y.max) {
				let mut product = 1;
				for i in range(0u, 4) {
					match dir {
						Horizontal => product *= *v.get(y).get(x+i),
						Vertical   => product *= *v.get(y+i).get(x),
						UpRight    => product *= *v.get(y-i).get(x+i),
						DownRight  => product *= *v.get(y+i).get(x+i)
					}
				}
				if product > *max { *max = product; }
			}
		}
	};

	let mut max = 0;

	/* Left to right */
	check(Range {min: 0, max: 17}, Range{min: 0, max: 20}, Horizontal, &mut max);

	/* Downwards */
	check(Range {min: 0, max: 20}, Range{min: 0, max: 17}, Vertical, &mut max);

	/* Up/right diagonally */
	check(Range {min: 0, max: 16}, Range{min: 3, max: 20}, UpRight, &mut max);

	/* Down/right diagonally */
	check(Range {min: 0, max: 16}, Range{min: 0, max: 16}, DownRight, &mut max);

	println!("Max = {}", max);
}
