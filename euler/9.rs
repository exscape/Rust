fn main() {
	for a in range(1,1000) {
		for b in range(a, 1000) {
			let c = 1000 - a - b;
			if a*a + b*b == c*c {
				println!("Match! {}*{}*{} = {}", a, b, c, a*b*c);
			}
		}
	}
}
