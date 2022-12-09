use std::collections::HashMap;

struct Board {
	knots: Vec<(isize, isize)>,
	tail_visited: HashMap<(isize, isize), usize>,
	min: (isize, isize),
	max: (isize, isize),
	steps: usize,
	num_knots: usize,
}

impl Board {
	fn new(num_knots: usize) -> Board {
		let mut board: Board = Board {
			knots: vec![(0, 0); num_knots],
			tail_visited: HashMap::new(),
			min: (0, 0),
			max: (0, 0),
			steps: 0,
			num_knots,
		};

		board.tail_visited.insert(board.knots[num_knots - 1], 1);

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
		let first_knot: (isize, isize) = self.knots[0];
		let last_knot: (isize, isize) = self.knots[self.num_knots - 1];

		*self.tail_visited.entry(last_knot).or_insert(0) += 1;

		self.min.0 = if first_knot.0 < self.min.0 { first_knot.0 } else { self.min.0 };
		self.min.1 = if first_knot.1 < self.min.1 { first_knot.1 } else { self.min.1 };
		self.max.0 = if first_knot.0 > self.max.0 { first_knot.0 } else { self.max.0 };
		self.max.1 = if first_knot.1 > self.max.1 { first_knot.1 } else { self.max.1 };

		self.steps += 1;
	}

	fn step(&mut self, direction: &str, steps: usize) {
		let delta: (isize, isize) = self.get_delta(direction);

		for _ in 0 .. steps {
			self.knots[0].0 += delta.0;
			self.knots[0].1 += delta.1;

			for knot_index in 1 .. self.num_knots {
				let knot_0: (isize, isize) = self.knots[knot_index - 1];
				let knot_1: (isize, isize) = self.knots[knot_index];

				let diff: (isize, isize) = (knot_0.0 - knot_1.0, knot_0.1 - knot_1.1);
			
				if diff.0.abs() > 1 || diff.1.abs() > 1 {
					if diff.0.abs() > 0 {
						self.knots[knot_index].0 += diff.0 / diff.0.abs();
					}

					if diff.1.abs() > 0 {
						self.knots[knot_index].1 += diff.1 / diff.1.abs();
					}
				}
			}

			self.update_visited();
		}
	}

	fn _print(&self, padding: isize) {
		let mut output = format!("Game Board: steps: {}, min: {}, {}, max: {}, {}\n",
								  self.steps, self.min.0, self.min.1, self.max.0, self.max.1);

		for y in self.min.1 - padding .. self.max.1 + padding + 1 {
			for x in self.min.0 - padding .. self.max.0 + padding + 1 {
				let mut knot_visualized: bool = false;

				for i in (0 .. self.num_knots).rev() {
					if self.knots[i].0 == x && self.knots[i].1 == y {
						let mut knot_visual: String = format!("{}", i);

						if i == 0 {
							knot_visual = "H".to_string();
						} else if i == self.num_knots - 1 {
							knot_visual = "T".to_string();
						}

						output.push_str(&format!("{}", knot_visual));
						knot_visualized = true;
					}
				}

				if !knot_visualized {
					if x == 0 && y == 0 {
						output.push_str("s");
					} else {
						output.push_str(".");
					}
				}
			}

			output.push_str("\n");
		}

		println!("{}", output);
	}
}

pub fn part1(input: String) {
	let mut board: Board = Board::new(2);

	for line in input.lines() {
		let (direction, steps_str) = line.split_once(" ").unwrap();
		let steps: usize = steps_str.parse::<usize>().unwrap();

		board.step(direction, steps);
	}

	println!("{}", board.tail_visited.keys().len());
}

pub fn part2(input: String) {
	let mut board: Board = Board::new(10);

	for line in input.lines() {
		let (direction, steps_str) = line.split_once(" ").unwrap();
		let steps: usize = steps_str.parse::<usize>().unwrap();

		board.step(direction, steps);
	}

	println!("{}", board.tail_visited.keys().len());
}
