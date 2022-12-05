use regex::Regex;

pub fn part1(input: String) {
	let mut parsing_crates: bool = true;
	let mut crates: Vec<Vec<char>> = vec![];

	for line in input.lines() {
		if parsing_crates {
			for i in 0 .. line.len() {
				if i > 0 && (i - 1) % 4 == 0 {
					let id = line.chars().nth(i).unwrap();
					let index = (i - 1) / 4;

					if index + 1 > crates.len() {
						crates.push(vec![]);
					}

					if id.is_ascii_uppercase() {
						crates[index].push(id);
					}
				}
			}
		} else {
			let re = Regex::new(r"move ([0-9]+) from ([0-9]) to ([0-9])").unwrap();
			for captures in re.captures_iter(line) {
				let number: usize = captures[1].parse::<usize>().unwrap();
				let from: usize = &captures[2].parse::<usize>().unwrap() - 1;
				let to: usize = &captures[3].parse::<usize>().unwrap() - 1;

				for _ in 0 .. number {
					let id = crates[from].pop().unwrap();
					crates[to].push(id);
				}
			}
		}

		if line == "" {
			parsing_crates = false;

			for i in 0 .. crates.len() {
				crates[i].reverse();
			}
		}
	}

	for i in 0 .. crates.len() {
		println!("{:?}", crates[i].last().unwrap());
	}
}

pub fn part2(input: String) {
	let mut parsing_crates: bool = true;
	let mut crates: Vec<Vec<char>> = vec![];

	for line in input.lines() {
		if parsing_crates {
			for i in 0 .. line.len() {
				if i > 0 && (i - 1) % 4 == 0 {
					let id = line.chars().nth(i).unwrap();
					let index = (i - 1) / 4;

					if index + 1 > crates.len() {
						crates.push(vec![]);
					}

					if id.is_ascii_uppercase() {
						crates[index].push(id);
					}
				}
			}
		} else {
			let re = Regex::new(r"move ([0-9]+) from ([0-9]) to ([0-9])").unwrap();
			for captures in re.captures_iter(line) {
				let number: usize = captures[1].parse::<usize>().unwrap();
				let from: usize = &captures[2].parse::<usize>().unwrap() - 1;
				let to: usize = &captures[3].parse::<usize>().unwrap() - 1;

				let mut temp_crates: Vec<char> = vec![];
				for _ in 0 .. number {
					let id = crates[from].pop().unwrap();
					temp_crates.push(id);
				}

				for _ in 0 .. number {
					let id = temp_crates.pop().unwrap();
					crates[to].push(id);
				}
			}
		}

		if line == "" {
			parsing_crates = false;

			for i in 0 .. crates.len() {
				crates[i].reverse();
			}
		}
	}

	for i in 0 .. crates.len() {
		println!("{:?}", crates[i].last().unwrap());
	}
}
