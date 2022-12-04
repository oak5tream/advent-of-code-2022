pub fn part1(input: String) {
	let mut sum: usize = 0;

	for line in input.lines() {
		let mut pairs = line.split(",");
		let mut p0 = pairs.next().unwrap().split("-");
		let mut p1 = pairs.next().unwrap().split("-");

		let s0 = (p0.next().unwrap().parse::<usize>().unwrap(), p0.next().unwrap().parse::<usize>().unwrap());
		let s1 = (p1.next().unwrap().parse::<usize>().unwrap(), p1.next().unwrap().parse::<usize>().unwrap());

		if (s0.0 <= s1.0 && s0.1 >= s1.1) || (s1.0 <= s0.0 && s1.1 >= s0.1) {
			sum += 1;
		}
	}

	println!("{}", sum);
}

pub fn part2(input: String) {
	let mut sum: usize = 0;

	for line in input.lines() {
		let mut pairs = line.split(",");
		let mut p0 = pairs.next().unwrap().split("-");
		let mut p1 = pairs.next().unwrap().split("-");

		let s0 = (p0.next().unwrap().parse::<usize>().unwrap(), p0.next().unwrap().parse::<usize>().unwrap());
		let s1 = (p1.next().unwrap().parse::<usize>().unwrap(), p1.next().unwrap().parse::<usize>().unwrap());

		for id in s0.0 .. s0.1 + 1 {
			if id >= s1.0 && id <= s1.1 {
				sum += 1;
				break;
			}
		}
	}

	println!("{}", sum);
}
