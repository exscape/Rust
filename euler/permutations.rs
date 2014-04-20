#![crate_id = "permutations#0.01"]

//! Lexicographic permutations for `std::vec::Vec`.

use std::{fmt, str};

//
// TODO: prev_permutation
// TODO: try to avoid code duplication in the above
// TODO: trait on Vec<T> instead of function
// TODO: extensive testing
// TODO: clean up code
//

/// Mutates the vector to the next lexicographical permutation.
///
/// Returns `true` if successful, `false` if input is already at the last permutation.
///
/// # Example
///
/// ```rust
/// use permutations::next_permutation;
/// let mut v = vec!(0, 1, 2);
/// next_permutation(&mut v);
/// assert_eq!(v, vec!(0, 2, 1));
/// next_permutation(&mut v);
/// assert_eq!(v, vec!(1, 0, 2));
/// ```
pub fn next_permutation<T : Ord>(v: &mut Vec<T>) -> bool {
	if v.len() < 2 { return false; } /* there is only 1 permutation each for these cases, so we are at the last */

	/* 
	 * Step 1: 
     * Find largest index i such that array[i−1] < array[i]. If no such index exists, the permutation is the last permutation.
	 */
	let mut i : uint = 0;
	let mut last : bool = true;

	for x in range(1, v.len()).rev() {
		i = x; // TODO!
		if v.get(i - 1) < v.get(i) { last = false; break; }
	}

	if last {
		/* This is the last permutation. */
		return false;
	}

	/*
	 * Step 2:
	 * Find largest index j such that j ≥ i and array[j] > array[i − 1].
	 */
	assert!(v.get(i-1) < v.get(i)); // pivot must be smaller than first part of suffix
	let mut j : uint = 0;
	let mut found : bool = false;
	for x in range(i, v.len()).rev() {
		j = x; // TODO!
		assert!(i > 0);
		if v.get(j) > v.get(i-1) { found = true; break; }
	}
	assert_eq!(found, true);

	/* Step 3: Swap array[j] and array[i − 1]. */
	v.as_mut_slice().swap(i-1, j);

	/* Step 4: Reverse the suffix starting at array[i]. */
	v.mut_slice_from(i).reverse();

	true
}

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
	impl fmt::Show for TS {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

	// TODO: what more can we test?
}


fn main() {
	let mut v : Vec<uint> = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

	println!("PERMUTATION: {}", v);

	for _ in range(0, 999999) {
		 next_permutation(&mut v);
	}
	assert_eq!(v, vec!(2, 7, 8, 3, 9, 1, 5, 4, 6, 0));
	println!("{}", v);


	let s = "ABCD";
	let mut t = s.chars().collect::<Vec<char>>();
	println!("String:");
	println!("{}", s);
	while next_permutation(&mut t) {
		println!("{}", str::from_chars(t.as_slice()));
	}
}
