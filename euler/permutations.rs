#![crate_id = "permutations#0.01"]

//! Lexicographic permutations for `std::vec::Vec`.

extern crate std;
extern crate test;

//
// TODO: first of all, clean up the code
//
//
// TODO: prev_permutation
// TODO: try to avoid code duplication in the above
// TODO: trait on Vec<T> instead of function
// TODO: extensive testing
// TODO: clean up code
//

/*
/// Mutates the vector to the next lexicographic permutation.
///
/// Returns `true` if successful, `false` if input is already at the last permutation. TODO
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
*/

pub struct LexicographicPermutations<T> {
	v: Vec<T>
}

trait LexicographicPermutationsTrait<T: Clone> { // TODO: rename
	fn lexicographic_permutations(self) -> LexicographicPermutations<T>;
}

impl<'a, T: Clone + Ord> LexicographicPermutationsTrait<T> for &'a [T] {
	fn lexicographic_permutations(self) -> LexicographicPermutations<T> {
		LexicographicPermutations { v: Vec::from_slice(self) }
	}
}

impl<T: Clone + Ord> Iterator<Vec<T>> for LexicographicPermutations<T> {
	fn next(&mut self) -> Option<Vec<T>> {
		let ret = self.v.clone();
		match next_permutation(&mut self.v) {
			true => Some(ret),
		 	false => None // TODO: this misses the very last permutation
		}
	}
}

pub fn next_permutation<T : Ord>(v: &mut Vec<T>) -> bool {
	if v.len() < 2 { return false; } /* there is only 1 permutation each for these cases, so we are at the last */

	/* 
	 * Step 1: 
     * Find largest index i such that array[i−1] < array[i]. If no such index exists, the permutation is the last permutation.
	 */
	let mut last : bool = true;

	let mut i : uint = v.len() - 1;
	while i > 0 {
		if v.get(i - 1) < v.get(i) { last = false; break; }
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
	let mut j = v.len() - 1;
	assert!(i > 0);
	while j > 0 {
		if v.get(j) > v.get(i-1) { break; }
		j -= 1;
	}

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
fn bench_without_iterator(b: &mut test::test::Bencher) {
	let mut v = vec!(0, 1, 2, 3, 4, 5, 6, 7);
	b.iter(|| { while next_permutation(&mut v) { } });
}

#[bench]
fn bench_with_iterator(b: &mut test::test::Bencher) {
	let v = vec!(0, 1, 2, 3, 4, 5, 6, 7);
	b.iter(|| { for _ in v.as_slice().lexicographic_permutations() { } });
}

#[allow(dead_code)]
fn main() {
	let mut v = vec!(1,2,3, 4, 5);

	loop {
		println!("{} ", v);
		if !next_permutation(&mut v) { break; }
	}
}
