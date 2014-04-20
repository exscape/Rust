use std::{fmt, str};

/// Mutates the vector to the next lexographical permutation.
///
/// Returns `true` if successful, `false` if input is already at the last permutation.
///
/// # Example
///
/// ```rust
/// let mut v = vec!(0, 1, 2);
/// permute(&mut v); // TODO
/// assert_eq!(v, vec!(0, 2, 1);
/// permute(&mut v);
/// assert_eq!(v, vec!(1, 0, 2);
/// ```
///
/// TODO: trait on Vec<T> instead of function
/// TODO: extensive testing
/// TODO: clean up code
fn next_permutation<T : Ord>(v: &mut Vec<T>) -> bool {
	if v.len() < 2 { return false; } /* there is only 1 permutation each for these cases, so we are at the last */

	/* 
	 * Step 1: 
     * Find largest index i such that array[i−1] < array[i]. If no such index exists, the permutation is the last permutation.
	 */
	let mut i : uint = 0;
	let mut last : bool = true;

//	println!("above loop; v.len = {}", v.len());
	for x in range(1, v.len()).rev() {
		i = x; // ffs
//		println!("in loop: i = {}, v.len() = {}", i, v.len());
		if v.get(i - 1) < v.get(i) { /*println!("found v[i-1] <v[i] ({} < {}) at i = {}", v.get(i-1), v.get(i), i); */last = false; break; }
	}

	if last {
		/* This is the last permutation. */
//		println!("last==true, returning false");
		return false;
	}

//	println!("i = {}", i);

	/*
	 * Step 2:
	 * Find largest index j such that j ≥ i and array[j] > array[i − 1].
	 */
	assert!(v.get(i-1) < v.get(i)); // pivot must be smaller than first part of suffix
	let mut j : uint = 0;
	let mut found : bool = false;
	for x in range(i, v.len()).rev() {
		j = x;
		assert!(i > 0);
		if v.get(j) > v.get(i-1) { found = true; break; }
	}
	assert_eq!(found, true);

//	println!("i: {}, j: {}", i, j);

	/* Step 3: Swap array[j] and array[i − 1]. */
	v.as_mut_slice().swap(i-1, j);

	/* Step 4: Reverse the suffix starting at array[i]. */
	v.mut_slice_from(i).reverse();

	true
}

#[test]
fn test_permutations() {
	let mut v : Vec<uint> = vec!(1, 2, 3, 4, 5);
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 3, 5, 4));
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 4, 3, 5));
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 4, 5, 3));
	next_permutation(&mut v);
	assert_eq!(v, vec!(1, 2, 5, 3, 4));
	// TODO: plenty of more tests, including empty, 1-length, 2-length etc. vectors of vastly different types
}

struct TestStruct { big: [uint, ..128], small: Option<uint> }

impl Ord for TestStruct {
	fn lt(&self, other: &TestStruct) -> bool {
		return self.small.unwrap() < other.small.unwrap();
	}
}
impl Eq for TestStruct {
	fn eq(&self, other: &TestStruct) -> bool {
		return self.small.unwrap() == other.small.unwrap();
	}
}
impl fmt::Show for TestStruct {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "{}", self.small.unwrap())
	}
}

fn main() {
	let mut v : Vec<uint> = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

	println!("PERMUTATION: {}", v);

	for _ in range(0, 999999) {
		 next_permutation(&mut v);
	}
	assert_eq!(v, vec!(2, 7, 8, 3, 9, 1, 5, 4, 6, 0));
	println!("{}", v);

	let mut v2 : Vec<TestStruct> = Vec::new();

	for i in range(0u, 3) {
		v2.push(TestStruct { big: [0, ..128], small: Some(i) });
	}

	println!("Fat struct:");
	println!("{}", v2);
	while next_permutation(&mut v2) {
		println!("{}", v2);
	}

	let s = "ABCD";
	let mut t = s.chars().collect::<Vec<char>>();
	println!("String:");
	println!("{}", s);
	while next_permutation(&mut t) {
		println!("{}", str::from_chars(t.as_slice()));
	}
}
