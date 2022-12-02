pub fn part1(input: String) {
	let lines: Vec<String> = input
		.lines()
		.map(|line| line.parse::<String>().unwrap())
		.collect();

	let mut score: usize = 0;

	for line in lines.iter() {
		let s: &str = &*line;

		match s {
			"A X" => score += 1 + 3,
			"A Y" => score += 2 + 6,
			"A Z" => score += 3 + 0,
			"B X" => score += 1 + 0,
			"B Y" => score += 2 + 3,
			"B Z" => score += 3 + 6,
			"C X" => score += 1 + 6,
			"C Y" => score += 2 + 0,
			"C Z" => score += 3 + 3,
			_ => panic!(),
		}
	}

	println!("{}", score);
}

pub fn part2(input: String) {
	let lines: Vec<String> = input
		.lines()
		.map(|line| line.parse::<String>().unwrap())
		.collect();

	let mut score: usize = 0;

	for line in lines.iter() {
		let s: &str = &*line;

		match s {
			"A X" => score += 3 + 0,
			"A Y" => score += 1 + 3,
			"A Z" => score += 2 + 6,
			"B X" => score += 1 + 0,
			"B Y" => score += 2 + 3,
			"B Z" => score += 3 + 6,
			"C X" => score += 2 + 0,
			"C Y" => score += 3 + 3,
			"C Z" => score += 1 + 6,
			_ => panic!(),
		}
	}

	println!("{}", score);
}
