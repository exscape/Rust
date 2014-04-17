extern crate exscape;
extern crate sync;
use exscape::is_palindrome_num;

fn inner_loop(i: uint) -> uint {
	let mut local_max : uint = 0;
	for j in range(i, 1000u) {
		if is_palindrome_num(i*j) && i*j > local_max {
			local_max = i*j;
		}
	}

	local_max
}

fn main() {
	let mut max : uint = 0;

    let mut futures = Vec::new();
	for i in range(100u, 1000) { futures.push(sync::Future::spawn(proc() { inner_loop(i) })) }

    for ft in futures.mut_iter()  {
		let task_max = ft.get();
		if task_max > max { max = task_max; }
    }

	println!("Max = {}", max);
}
