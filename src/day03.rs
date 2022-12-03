use itertools::Itertools;

pub fn part1(input: String) {
	let mut sum: usize = 0;

	for line in input.lines() {
		let (p0, p1) = line.split_at(line.len() / 2);

		for c in p0.chars().sorted().unique() {
			if p1.contains(c) {
				let val = c as usize;
				let prio: usize = if val > 96 { val - 96 } else { val - 38 };

				sum += prio;
			}
		}
	}

	println!("{}", sum);
}

pub fn part2(input: String) {
	let lines: Vec<String> = input
		.lines()
		.map(|line| line.parse::<String>().unwrap())
		.collect();

	let mut sum: usize = 0;

	for i in 0 .. (lines.len() / 3) {
		let l0: String = lines[i * 3 + 0].clone();
		let l1: String = lines[i * 3 + 1].clone();
		let l2: String = lines[i * 3 + 2].clone();

		for c in l0.chars().sorted().unique() {
			if l1.contains(c) && l2.contains(c) {
				let val = c as usize;
				let prio: usize = if val > 96 { val - 96 } else { val - 38 };

				sum += prio;
			}
		}
	}

	println!("{}", sum);
}
