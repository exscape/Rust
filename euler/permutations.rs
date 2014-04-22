#![crate_id = "permutations#0.01"]

//! Lexicographic permutations for mutable slices.

extern crate std;
extern crate test;

//
// TODO: first of all, clean up the code
//
//
// TODO: prev_permutation
// TODO: try to avoid code duplication in the above
// TODO: extensive testing

/// Trait implementing lexicographic permutations for mutable slices
pub trait LexicographicPermutations<T: Ord> { 
	fn next_permutation(&mut self) -> bool; 
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
	/// assert_eq!(v.as_slice(), & [0, 2, 1]);
	/// v.next_permutation();
	/// assert_eq!(v.as_slice(), & [1, 0, 2]);
	/// ```
	fn next_permutation(&mut self) -> bool {
		if self.len() < 2 { return false; } /* there is only 1 permutation each for these cases, so we are at the last */

		/* 
		 * Step 1: 
		 * Find largest index i such that array[i−1] < array[i]. If no such index exists, the permutation is the last permutation.
		 */
		let mut last : bool = true;

		let mut i : uint = self.len() - 1;
		while i > 0 {
			if self[i - 1] < self[i] { last = false; break; }
			i -= 1;
		}

		if last {
			/* This is the last permutation. */
			return false;
		}

		/*
		 * Step 2:
		 * Find largest index j such that j ≥ i and array[j] > array[i − 1].
		 */
		let mut j = self.len() - 1;
		assert!(i > 0);
		while j > 0 {
			if self[j] > self[i-1] { break; }
			j -= 1;
		}

		/* Step 3: Swap array[j] and array[i − 1]. */
		self.swap(i-1, j);

		/* Step 4: Reverse the suffix starting at array[i]. */
		self.mut_slice_from(i).reverse();

		true
	}
}
/*
#[test]
fn test_permutations() {
	/* Test simple permutations */
	let mut v : Vec<uint> = vec!(1, 2, 3, 4, 5);
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 3, 5, 4));
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 4, 3, 5));
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 4, 5, 3));
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 5, 3, 4));

	/* Test 0-length, 1-length and 2-length vectors */
	let mut v : Vec<int> = Vec::new();
	assert!(next_permutation(&mut v) == false);
	assert_eq!(v, Vec::new());
	v.push(1);
	assert!(next_permutation(&mut v) == false);
	assert_eq!(v, vec!(1));
	v.push(2);
	assert!(next_permutation(&mut v) == true);
	assert_eq!(v, vec!(2, 1));
	assert!(next_permutation(&mut v) == false);
	assert_eq!(v, vec!(2, 1));

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
}

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
#[allow(dead_code)]
fn main() {
	let mut v = &mut [1, 2, 3, 4, 5];

	loop {
		println!("{} ", v.as_slice());
		if !v.next_permutation() { break; }
	}
}
