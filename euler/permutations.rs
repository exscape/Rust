#![crate_id = "permutations#0.01"]

//! Lexicographic permutations for mutable slices.

extern crate std;
extern crate test;

/* for testing only */
extern crate rand;
use rand::{task_rng, Rng};

//
// TODO: first of all, clean up the code
//
// TODO: extensive testing

/// Trait implementing lexicographic permutations for mutable slices
pub trait LexicographicPermutations<T: Ord> {
	fn next_permutation(&mut self) -> bool;
	fn prev_permutation(&mut self) -> bool;
}

impl<'a, T: Ord> LexicographicPermutations<T> for &'a mut [T] { 
	/// Mutates the slice to the next lexicographic permutation. TODO: this text is not visible in the docs
	///
	/// Returns `true` if successful, `false` if input is already at the last permutation.
	///
	/// # Example
	///
	/// ```rust
	/// use permutations::LexicographicPermutations;
	/// let mut v = &mut [0, 1, 2];
	/// v.next_permutation();
	/// assert_eq!(v, &mut [0, 2, 1]);
	/// v.next_permutation();
	/// assert_eq!(v, &mut [1, 0, 2]);
	/// ```
	fn next_permutation(&mut self) -> bool {
		if self.len() < 2 { return false; } /* there is only 1 permutation each for these cases, so we are at the last */

		/*
		 * Step 1:
		 * Identify the longest, rightmost weakly decreasing part of the vector.
		 * If none exists, this is the last permutation.
		 */
		let mut i = self.len() - 1;
		while i > 0 && self[i-1] >= self[i] {
			i -= 1;
		}

		if i == 0 {
			/* This is the last permutation. */
			return false;
		}

		/*
		 * Step 2:
		 * Find largest index j such that j ≥ i and array[j] > array[i − 1].
		 */
		let mut j = self.len() - 1;
		while j >= i && self[j] <= self[i-1]  {
			j -= 1;
		}

		/* Step 3: Swap array[j] and array[i − 1]. */
		self.swap(j, i-1);

		/* Step 4: Reverse the suffix starting at array[i]. */
		self.mut_slice_from(i).reverse();

		true
	}

	fn prev_permutation(&mut self) -> bool {
		if self.len() < 2 { return false; } /* there is only 1 permutation each for these cases, so we are at the first */

		let mut i = self.len() - 1;
		while i > 0 && self[i-1] <= self[i] {
			i -= 1;
		}

		if i == 0 {
			/* This is the first permutation. */
			return false;
		}

		self.mut_slice_from(i).reverse();

		let mut j = self.len() - 1;
		while j >= i && self[j] < self[i-1]  {
			j -= 1;
		}
		// TODO: why does this always work, such that the assert never fails? can this be proven?

		// We only get here if:
		// 1) The vector is 2 or more elements
		// 2) The vector is NOT at the first permutation, i.e. something WILL be done to permute it
		// 3) The vector has been reversed from index 1 or higher (the entire thing is NEVER reversed) -- except for length two, which swap() does below


		assert!(j != self.len() - 1); /* otherwise the next line is out of bounds */

		self.swap(i-1, j+1);

		true
	}
}

#[test]
fn test_permutations() {
	/* Test simple permutations */
	let mut v : &mut[int] = &mut[1, 2, 3, 4, 5];
	assert!(v.prev_permutation() == false);
	assert!(v.next_permutation());
	assert_eq!(v, &mut[1, 2, 3, 5, 4]);
	assert!(v.prev_permutation());
	assert_eq!(v, &mut[1, 2, 3, 4, 5]);
	assert!(v.next_permutation());
	assert!(v.next_permutation());
	assert_eq!(v, &mut[1, 2, 4, 3, 5]);
	assert!(v.next_permutation());
	assert_eq!(v, &mut[1, 2, 4, 5, 3]);

	/* Test 0-length, 1-length and 2-length slices */
	let mut empty : &mut[int] = &mut[];
	assert!(empty.next_permutation() == false);
	assert_eq!(empty, &mut[]);
	assert!(empty.prev_permutation() == false);
	assert_eq!(empty, &mut[]);

	let mut small : &mut[int] = &mut[1, 2];

	assert!(small.prev_permutation() == false);
	assert_eq!(small, &mut[1, 2]);
	assert!(small.next_permutation());
	assert_eq!(small, &mut[2, 1]);
	assert!(small.next_permutation() == false);
	assert_eq!(small, &mut[2, 1]);
	assert!(small.prev_permutation());
	assert_eq!(small, &mut[1, 2]);
	assert!(small.prev_permutation() == false);
	assert_eq!(small, &mut[1, 2]);
	/*

	/* Test a custom struct */
	struct TS { big: [uint, ..4], small: Option<uint> }
	impl Ord for TS {
		fn lt(&self, other: &TS) -> bool {
			return self.small.unwrap() < other.small.unwrap();
		}
	}
	impl Eq for TS {
		fn eq(&self, other: &TS) -> bool {
			return self.small.unwrap() == other.small.unwrap();
		}
	}
	impl std::fmt::Show for TS {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(f.buf, "{}", self.small.unwrap())
		}
	}

	let mut v2 : Vec<TS> = Vec::new();

	for i in range(0u, 3) {
		v2.push(TS { big: [i, ..4], small: Some(i) });
	}

	next_permutation(&mut v2);
	assert_eq!(v2, vec!(TS{ big: [0, ..4], small: Some(0) }, TS{ big: [0, ..4], small: Some(2) }, TS{ big: [0, ..4], small: Some(1) }));
	next_permutation(&mut v2);
	assert_eq!(v2, vec!(TS{ big: [0, ..4], small: Some(1) }, TS{ big: [0, ..4], small: Some(0) }, TS{ big: [0, ..4], small: Some(2) }));

	// Project euler #24
	let mut v : Vec<uint> = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

	for _ in range(0, 999999) {
		 next_permutation(&mut v);
	}
	assert_eq!(v, vec!(2, 7, 8, 3, 9, 1, 5, 4, 6, 0));

	let s = "ABCD";
	let mut t = s.chars().collect::<Vec<char>>();
	next_permutation(&mut t);
	assert_eq!(t, vec!('A', 'B', 'D', 'C'));
	next_permutation(&mut t);
	assert_eq!(t, vec!('A', 'C', 'B', 'D'));
	next_permutation(&mut t);
	assert_eq!(t, vec!('A', 'C', 'D', 'B'));
	*/
}

/*
#[bench]
fn bench_without_iterator_small(b: &mut test::test::Bencher) {
		let mut v = range(0u,10u).collect::<Vec<uint>>(); 
		b.iter(|| { let mut i = 0; while i < 10000 { next_permutation(&mut v); i += 1; } });
}

#[bench]
fn bench_without_iterator_big(b: &mut test::test::Bencher) {
	let mut v = range(0u,1000000u).collect::<Vec<uint>>();
	b.iter(|| { let mut i = 0; while i < 10000 { next_permutation(&mut v); i += 1; } });
}

#[bench]
fn bench_with_iterator(b: &mut test::test::Bencher) {
	let v = range(0u,10u).collect::<Vec<uint>>();
	b.iter(|| { let mut i = 0; for _ in v.as_slice().lexicographic_permutations() { if i > 10000 { break; } i += 1; } });
}
*/

//#[test]
fn test_permutations_2(input: &mut &mut[int]) {
	// This was designed as a test for prev_permutation, after it was known that
	// next_permutation worked.
	let mut x = Vec::from_slice(*input);
	x.sort();
	let mut y = x.clone(); y.reverse();
	let mut forward = x.as_mut_slice();
	let mut backward = y.as_mut_slice();
	let mut results_forward : Vec<Vec<int>> = Vec::new();
	let mut results_backward : Vec<Vec<int>> = Vec::new();

	let mut i = 0;
	loop {
//	println!("loop 1: {}, it. {}", forward, i);
		i += 1;
		results_forward.push(Vec::from_slice(forward));
		if !forward.next_permutation() { break; }
	}

	let mut j = 0;
	loop {
//	println!("loop 2: {}, it. {}", backward, j);
		j += 1;
		results_backward.unshift(Vec::from_slice(backward)); /* slow, but for small inputs, that's unnoticeable */
		if !backward.prev_permutation() { break; }
	}
	assert_eq!(results_forward.len(), results_backward.len()); // TODO: remove this early test
	assert_eq!(results_forward, results_backward);
//	println!("Test pass for input {} in {}/{} iterations!", *input, i, j);
}

#[bench]
fn bench_my_permutations_bulk(b: &mut test::test::Bencher) {
	b.iter(|| { let mut data : &mut[int] = &mut[0, 1, 2, 3, 4, 5, 6]; while data.next_permutation() { } });
}

#[bench]
fn bench_rust_permutations_bulk(b: &mut test::test::Bencher) {
	b.iter(|| {	let mut data : &mut[int] = &mut[0, 1, 2, 3, 4, 5, 6]; for _ in data.permutations() { } } );
}

#[bench]
fn bench_my_permutations_2(b: &mut test::test::Bencher) {
	let mut data : &mut[int] = &mut[0, 0, 0, 1, 1, 1, 3, 3, 4, 5, 6, 7, 8, 8, 9];
	b.iter(|| { data.next_permutation(); });
}

#[bench]
fn bench_rust_permutations_2(b: &mut test::test::Bencher) {
	let mut data : &mut[int] = &mut[0, 0, 0, 1, 1, 1, 3, 3, 4, 5, 6, 7, 8, 8, 9];
	let mut p = data.permutations();
	b.iter(|| {	p.next(); });
}

fn infinite_loop_random_test() {
	let mut rng = task_rng();

	let mut num_passed : uint = 0;
	loop {
		let mut v : Vec<int> = Vec::new();
		// create a random vec, of random length
		for _ in range(0, rng.gen_range(3u, 8)) {
			v.push(rng.gen_range(0, 14));
		}

		v.sort();
		test_permutations_2(&mut v.as_mut_slice());
		num_passed += 1;
		println!("{} tests passed", num_passed);
	}
}

//#[test]
fn main() {
	// infinite_loop_random_test();

	// Benchmark; not sure I trust #[bench] (or trust myself to use it correctly),
	// so this is a double-check type deal.

	// This runs in about 12.85 seconds, for 12.85 ns/iteration, on my 2011 2.2 GHz Sandy Bridge MBP!
	// #[bench] gives about 14 ns/iter, so that seems about right.

	let mut i = 0u;
	let mut data : &mut[int] = &mut[0, 0, 0, 1, 1, 1, 3, 3, 4, 5, 6, 7, 8, 8, 9];
	while i < 100000000u {
		data.next_permutation();
		i += 1;
	}
	println!("final iteration: {}", data);
/*
	let mut i = 0u;
	let mut data = &mut[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	while i < 999999u { data.next_permutation(); i+=1; }

	*/

	/*
	let mut x : &mut[int] = &mut[1, 2, 3, 4, 5];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[0];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[1, 0];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[0, 0, 3, 2, 5, 1, 4];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[0, 0, 0, 0];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[0, 0, 0, 1];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[0, 1, 1, 2, 2, 3, 5];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[9, 3, 1, 4, 1, 4, 4];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[3, 3, 3, 3, 3, 1, 3];
	test_permutations_2(&mut x);
	let mut x : &mut[int] = &mut[1, 2, 3, 4, 3, 2, 1, 3, 1];
	test_permutations_2(&mut x);
	*/
}
