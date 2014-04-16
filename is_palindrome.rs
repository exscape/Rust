fn is_palindrome(s: &str) -> bool {
	let filtered : ~str = s.chars().filter_map(|c| {
			match c.is_alphanumeric() {
				true => Some(c.to_lowercase()),
				false => None
			}
		}).collect();

	filtered == filtered.chars_rev().collect()
}

fn is_palindrome_num(n: uint) -> bool {
	is_palindrome(n.to_str())
}

fn main() {
	println!("Strings:");
	let strings = ["abba", "anna", "12321", "bob", "too hot to hoot", "If I had a hi-fi", "arne", "test", "inte palindrom", "no siree"];
	for s in strings.iter() {
		println!("{} for {}", is_palindrome(*s), *s);
	}

	let numbers = [1, 12321, 6006, 666, 101, 9009, 9009009, 30403, 123, 9013, 582, 402, 16026, 492855];
	println!("Numbers:");
	for &s in numbers.iter() {
		println!("{} for {}", is_palindrome_num(s), s);
	}
}
