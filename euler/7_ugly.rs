/* 2014-04-15, Rust 0.11-pre */

fn isprime(n: uint) -> bool {
	if n > 2 && n % 2 == 0 { return false; }
	else if n == 2 { return true; }

	let mut d = 3u;

	while d*d <= n { /* while d <= sqrt(n) */
		if n % d == 0 {
			return false;
		}
		d += 2;
	}

	return true; /* just "true" doesn't look very nice */
}

fn main() {
	/* This is really ugly, but I'm not sure how to write it prettier such at it doesn't check 4, 6, 8 etc. */
	let mut n = 3u;
	let mut count = 1;
	loop {
		if isprime(n) {
			count += 1;
			if count >= 10001 { break; }
		}
		n += 2;
	}

	println!("Answer: 10001st prime is {}", n);
}
