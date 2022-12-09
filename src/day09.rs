use std::collections::HashMap;

/*#[repr((isize, isize))]
enum Direction {
	Up = (0, -1),
	Down = (0, 1),
	Left = (-1, 0),
	Right = (1, 0),
}*/

struct Board {
	head: (isize, isize),
	tail: (isize, isize),
	head_visited: HashMap<(isize, isize), usize>,
	tail_visited: HashMap<(isize, isize), usize>,
	min: (isize, isize),
	max: (isize, isize),
	steps: usize,
}

impl Board {
	fn new() -> Board {
		let mut board: Board = Board {
			head: (0, 0),
			tail: (0, 0),
			head_visited: HashMap::new(),
			tail_visited: HashMap::new(),
			min: (0, 0),
			max: (0, 0),
			steps: 0,
		};

		board.head_visited.insert(board.head, 1);
		board.head_visited.insert(board.tail, 1);

		board
	}

	fn get_delta(&self, direction: &str) -> (isize, isize) {
		return match direction {
			"U" => (0, -1),
			"D" => (0, 1),
			"L" => (-1, 0),
			"R" => (1, 0),
			_ => panic!(),
		}
	}

	fn update_visited(&mut self) {
			*self.head_visited.entry(self.head).or_insert(0) += 1;
			*self.tail_visited.entry(self.tail).or_insert(0) += 1;

			self.min.0 = if self.head.0 < self.min.0 { self.head.0 } else { self.min.0 };
			self.min.1 = if self.head.1 < self.min.1 { self.head.1 } else { self.min.1 };
			self.max.0 = if self.head.0 > self.max.0 { self.head.0 } else { self.max.0 };
			self.max.1 = if self.head.1 > self.max.1 { self.head.1 } else { self.max.1 };

			self.steps += 1;
	}

	fn step(&mut self, direction: &str, steps: usize) {
		let delta: (isize, isize) = self.get_delta(direction);

//		println!("Stepping {} {} times", direction, steps);
		for _ in 0 .. steps {
			self.head.0 += delta.0;
			self.head.1 += delta.1;

//			println!("Before tail update - head: {:?}, tail: {:?}", self.head, self.tail);

			let diff: (isize, isize) = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
			
			if diff.0.abs() > 1 || diff.1.abs() > 1 {
				if diff.0.abs() > 0 {
					self.tail.0 += diff.0 / diff.0.abs();
				}

				if diff.1.abs() > 0 {
					self.tail.1 += diff.1 / diff.1.abs();
				}
			}

			self.update_visited();

//			println!("After tail update - head: {:?}, tail: {:?}", self.head, self.tail);
//			self._print(2);
		}
	}

	fn _print(&self, padding: isize) {
		let mut output = format!("Game Board: steps: {}, min: {}, {}, max: {}, {}\n",
								  self.steps, self.min.0, self.min.1, self.max.0, self.max.1);

		for y in self.min.1 - padding .. self.max.1 + padding + 1 {
			for x in self.min.0 - padding .. self.max.0 + padding + 1 {
				if self.head.0 == x && self.head.1 == y {
					output.push_str("H");
				} else if self.tail.0 == x && self.tail.1 == y {
					output.push_str("T");
				} else if x == 0 && y == 0 {
					output.push_str("s");
				} else {
					output.push_str(".");
				}
			}

			output.push_str("\n");
		}

//		println!("{}", output);
	}
}

pub fn part1(input: String) {
	let mut board: Board = Board::new();

//	board._print(2);

	for line in input.lines() {
		let (direction, steps_str) = line.split_once(" ").unwrap();
		let steps: usize = steps_str.parse::<usize>().unwrap();

		board.step(direction, steps);
	}

	println!("{}", board.tail_visited.keys().len());
}

pub fn part2(_input: String) {
}
