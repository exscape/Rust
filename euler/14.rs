/* 2014-04-18, Rust 0.11-pre-nightly */

fn collatz_len(mut n: uint) -> uint {
	let mut count = 1u;

	while n != 1 {
		n = match n % 2 { 0 => n/2, _ => 3*n + 1 };
		count += 1;
	}

	count
}

struct Max {num: uint, len: uint}

fn main() {
	let mut max = Max {num: 0, len: 0};
	for i in range(1u, 1000000) {
		let len = collatz_len(i);
		if len > max.len { max.len = len; max.num = i; }
	}

	println!("Max: {} @ {} terms", max.num, max.len);
}
