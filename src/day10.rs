struct Instruction {
	cycles: usize,
	increase: isize,
}

fn get_instruction(line: &str) -> Instruction {
	let mut instruction: Instruction = Instruction {
		cycles: 0,
		increase: 0,
	};

	if line.starts_with("noop") {
		instruction.cycles = 1;
		instruction.increase = 0;
	} else if line.starts_with("addx") {
		let (_, increase) = line.split_once(" ").unwrap();
		instruction.cycles = 2;
		instruction.increase = increase.parse::<isize>().unwrap();
	}

	instruction
}

fn solve(input: String) -> (isize, Vec<bool>) {
	let mut cycles: usize = 0;
	let mut x: isize = 1;
	let mut instructions: Vec<Instruction> = vec![];
	let mut current_instruction: Instruction = get_instruction("noop");
	let mut execute_complete_cycle: usize = 0;
	let mut running: bool = true;
	let mut crt: Vec<bool> = vec![false; 240];

	let mut signal_strength: isize = 0;

	for line in input.lines() {
		instructions.insert(0, get_instruction(line));
	}

	while running {
		if cycles >= execute_complete_cycle {
			current_instruction = instructions.pop().unwrap();
			execute_complete_cycle = cycles + current_instruction.cycles;
		}

		cycles += 1;

		if (cycles as isize - 20) % 40 == 0 {
			signal_strength += cycles as isize * x;
		}

		let sprite = x - 1 .. x + 2;
		let c: isize = (cycles as isize - 1) % 40;

		if sprite.contains(&c) {
			crt[cycles - 1] = true;
		}

		if cycles >= execute_complete_cycle {
			x += current_instruction.increase;

			if instructions.len() == 0 {
				running = false;
			}
		}
	}

	(signal_strength, crt)
}

pub fn part1(input: String) {
	let (signal_strength, _) = solve(input);

	println!("{}", signal_strength);
}

pub fn part2(input: String) {
	let (_, crt) = solve(input);

	for y in 0 .. 6 {
		let mut output = "".to_string();

		for x in 0 .. 40 {
			output.push_str(if crt[y * 40 + x] { "#" } else { "." });
		}

		println!("{}", output);
	}
}
