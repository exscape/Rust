extern crate gmp; // Requires the rust-gmp crate
extern crate exscape;
use exscape::fac_mpz;
use gmp::Mpz;
use std::iter::range_inclusive;

/// n choose r using GMP's Mpz type, whose size is limited only by memory usage.
fn choose_mpz(n: uint, r: uint) -> Mpz {
	if r > n { return std::num::zero(); }
	else if r == n { return std::num::one() }

	fac_mpz(n) / (fac_mpz(r) * fac_mpz(n - r))
}

fn main() {
	let mut count = 0;
	let million : Mpz = FromPrimitive::from_u64(1_000_000u64).unwrap();
	for n in range_inclusive(1u, 100) {
		for r in range_inclusive(1u, 100) {
			if choose_mpz(n, r) > million {
				count += 1;
			}
		}
	}

	println!("Answer: {}", count);
}
