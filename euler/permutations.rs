#![crate_id = "permutations#0.01"]

//! Lexicographic permutations for mutable slices.

extern crate std;
extern crate test;

/* for testing only */
extern crate rand;
use rand::{task_rng, Rng};

/// Trait implementing lexicographic permutations for mutable slices
///
/// # Example
///
/// ```rust
/// use permutations::LexicographicPermutations;
/// let mut v =  &mut [1, 2, 3];
/// // Prints all permutations, including the original, in lexicographical order
/// // Note: To print *all* permutations, the input must be in lexicographical order to begin with.
/// loop {
///     println!("{}", v);
///     if !v.next_permutation2() { break; }
/// }
///
/// ```
pub trait LexicographicPermutations<T: Ord> {
	/// Mutates the slice to the next lexicographic permutation.
	///
	/// Returns `true` if successful, `false` if input is already at the last-ordered permutation.
	///
	/// # Example
	///
	/// ```rust
	/// use permutations::LexicographicPermutations;
	/// let mut v = &mut [0, 1, 2];
	/// v.next_permutation2();
	/// assert_eq!(v, &mut [0, 2, 1]);
	/// v.next_permutation2();
	/// assert_eq!(v, &mut [1, 0, 2]);
	/// ```
	fn next_permutation2(self) -> bool;

	/// Mutates the slice to the previous lexicographic permutation.
	///
	/// Returns `true` if successful, `false` if input is already at the first-ordered permutation.
	///
	/// # Example
	///
	/// ```rust
	/// use permutations::LexicographicPermutations;
	/// let mut v = &mut [1, 0, 2];
	/// v.prev_permutation2();
	/// assert_eq!(v, &mut [0, 2, 1]);
	/// v.prev_permutation2();
	/// assert_eq!(v, &mut [0, 1, 2]);
	/// ```
	fn prev_permutation2(self) -> bool;
}

impl<'a, T: Ord> LexicographicPermutations<T> for &'a mut [T] {
	fn next_permutation2(self) -> bool {
		// These cases only have 1 permutation each, so we can't do anything.
		if self.len() < 2 { return false; }

		// Step 1: Identify the longest, rightmost weakly decreasing part of the vector
		let mut i = self.len() - 1;
		while i > 0 && self[i-1] >= self[i] {
			i -= 1;
		}

		// If that is the entire vector, this is the last-ordered permutation.
		if i == 0 {
			return false;
		}

		// Step 2: Find the rightmost element larger than the pivot (i-1)
		let mut j = self.len() - 1;
		while j >= i && self[j] <= self[i-1]  {
			j -= 1;
		}

		// Step 3: Swap that element with the pivot
		self.swap(j, i-1);

		// Step 4: Reverse the weakly decreasing part
		self.mut_slice_from(i).reverse();

		true
	}

	fn prev_permutation2(self) -> bool {
		// These cases only have 1 permutation each, so we can't do anything.
		if self.len() < 2 { return false; }

		// Step 1: Identify the longest, rightmost weakly increasing part of the vector
		let mut i = self.len() - 1;
		while i > 0 && self[i-1] <= self[i] {
			i -= 1;
		}

		// If that is the entire vector, this is the first-ordered permutation.
		if i == 0 {
			return false;
		}

		// Step 2: Reverse the weakly increasing part
		self.mut_slice_from(i).reverse();

		// Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
		let mut j = self.len() - 1;
		while j >= i && self[j-1] < self[i-1]  {
			j -= 1;
		}

		// Step 4: Swap that element with the pivot
		self.swap(i-1, j);

		true
	}
}

#[test]
fn test_lexicographic_permutations_int() {
	/* Test simple permutations */
	let mut v : &mut[int] = &mut[1, 2, 3, 4, 5];
	assert!(v.prev_permutation2() == false);
	assert!(v.next_permutation2());
	assert_eq!(v, &mut[1, 2, 3, 5, 4]);
	assert!(v.prev_permutation2());
	assert_eq!(v, &mut[1, 2, 3, 4, 5]);
	assert!(v.next_permutation2());
	assert!(v.next_permutation2());
	assert_eq!(v, &mut[1, 2, 4, 3, 5]);
	assert!(v.next_permutation2());
	assert_eq!(v, &mut[1, 2, 4, 5, 3]);

	let mut v: &mut[int] = &mut[1, 0, 0, 0];
	assert!(v.next_permutation2() == false);
	assert!(v.prev_permutation2());
	assert_eq!(v, &mut[0, 1, 0, 0]);
	assert!(v.prev_permutation2());
	assert_eq!(v, &mut[0, 0, 1, 0]);
	assert!(v.prev_permutation2());
	assert_eq!(v, &mut[0, 0, 0, 1]);
	assert!(v.prev_permutation2() == false);
}

#[test]
fn test_lexicographic_permutations_empty_and_short() {
	/* Test 0-length, 1-length and 2-length slices */
	let mut empty : &mut[int] = &mut[];
	assert!(empty.next_permutation2() == false);
	assert_eq!(empty, &mut[]);
	assert!(empty.prev_permutation2() == false);
	assert_eq!(empty, &mut[]);

	let mut one_elem : &mut[int] = &mut[4];
	assert!(one_elem.prev_permutation2() == false);
	assert_eq!(one_elem, &mut[4]);
	assert!(one_elem.next_permutation2() == false);
	assert_eq!(one_elem, &mut[4]);

	let mut two_elem : &mut[int] = &mut[1, 2];
	assert!(two_elem.prev_permutation2() == false);
	assert_eq!(two_elem, &mut[1, 2]);
	assert!(two_elem.next_permutation2());
	assert_eq!(two_elem, &mut[2, 1]);
	assert!(two_elem.next_permutation2() == false);
	assert_eq!(two_elem, &mut[2, 1]);
	assert!(two_elem.prev_permutation2());
	assert_eq!(two_elem, &mut[1, 2]);
	assert!(two_elem.prev_permutation2() == false);
	assert_eq!(two_elem, &mut[1, 2]);
}

// Code below this line is just development-focused test code, not intended to be committed.

fn test_all_permutations(input: &mut &mut[int]) {
	// This was designed as a test for prev_permutation, after it was known that
	// next_permutation worked. It generates *and stores* all permutations of the input,
	// in two ways, and verifies that they generate the same result.
	// Only usable for inputs of length 10 or less, approximately. 
	// (That's 2*10! = 7.25 million permutations, each stored, each at least 80 bytes on 64-bit.)
	// Length 11 would be 2*11! = 79.8 million, meaning at LEAST 2*11!*11*8 = 6.5 GiB RAM used.
	let mut x = Vec::from_slice(*input);
	x.sort();
	let mut y = x.clone(); y.reverse();
	let mut forward = x.as_mut_slice();
	let mut backward = y.as_mut_slice();
	let mut results_forward : Vec<Vec<int>> = Vec::new();
	let mut results_backward : Vec<Vec<int>> = Vec::new();

	loop {
		results_forward.push(Vec::from_slice(forward));
		if !forward.next_permutation2() { break; }
	}

	loop {
		results_backward.push(Vec::from_slice(backward));
		if !backward.prev_permutation2() { break; }
	}

	results_backward.reverse();
	assert_eq!(results_forward.len(), results_backward.len()); /* makes for much better error output when this is the case */
	assert_eq!(results_forward, results_backward);
}

#[bench]
fn bench_lexicographic_permutations_bulk(b: &mut test::test::Bencher) {
	b.iter(|| { let mut data : &mut[int] = &mut[0, 1, 2, 3, 4, 5, 6]; while data.next_permutation2() { } });
}

#[bench]
fn bench_iterated_permutations_bulk(b: &mut test::test::Bencher) {
	b.iter(|| {	let mut data : &mut[int] = &mut[0, 1, 2, 3, 4, 5, 6]; for _ in data.permutations() { } } );
}

#[bench]
fn bench_lexicographic_permutations_one_iter(b: &mut test::test::Bencher) {
	let mut data : &mut[int] = &mut[0, 0, 0, 1, 1, 1, 3, 3, 4, 5, 6, 7, 8, 8, 9];
	b.iter(|| { data.next_permutation2(); });
}

#[bench]
fn bench_iterated_permutations_one_iter(b: &mut test::test::Bencher) {
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
		test_all_permutations(&mut v.as_mut_slice());
		num_passed += 1;
		println!("{} random tests passed", num_passed);
	}
}

fn main() {
    let mut x = ~[1i,2,3];
    loop {
        println!("{}", x);
        if !x.next_permutation2() { break; }
    }
}
