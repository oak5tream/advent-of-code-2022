use std::collections::HashMap;
use std::{thread, time::Duration};


struct Obstacle;

impl Obstacle {
	pub const ROCK: char = '#';
	pub const SAND: char = 'o';
}

struct Simulation {
	min: (isize, isize),
	max: (isize, isize),
	grid: HashMap<(isize, isize), char>,
	sand: (isize, isize),
	sand_units: usize,
	steps: usize,
	floor: isize,
}

impl Simulation {
	fn new(input: String, floor: bool) -> Simulation {
		let mut min: (isize, isize) = (isize::max_value(), isize::max_value());
		let mut max: (isize, isize) = (isize::min_value(), isize::min_value());
		let mut grid: HashMap<(isize, isize), char> = HashMap::new();

		for line in input.lines() {
			let mut prev_point: (isize, isize) = (0, 0);
			let points = line.split(" -> ");

			for point in points {
				let (x_str, y_str) = point.split_once(",").unwrap();
				let (x, y): (isize, isize) = (x_str.parse::<isize>().unwrap(), y_str.parse::<isize>().unwrap());

				min.0 = if x < min.0 { x } else { min.0 };
				min.1 = if y < min.1 { y } else { min.1 };
				max.0 = if x > max.0 { x } else { max.0 };
				max.1 = if y > max.1 { y } else { max.1 };

				if prev_point.0 != 0 && prev_point.1 != 0 {
					if prev_point.0 == x {
						for i in isize::min(prev_point.1, y) .. isize::max(prev_point.1, y) + 1 {
							if !grid.contains_key(&(x, i)) {
								grid.insert((x, i), Obstacle::ROCK);
							}
						}
					} else if prev_point.1 == y {
						for i in isize::min(prev_point.0, x) .. isize::max(prev_point.0, x) + 1 {
							if !grid.contains_key(&(i, y)) {
								grid.insert((i, y), Obstacle::ROCK);
							}
						}
					} else {
						panic!("Path not a straight line!");
					}
				}

				prev_point = (x, y);
			}
		}

		let mut simulation: Simulation = Simulation {
			min,
			max,
			grid,
			sand: (500, 0),
			sand_units: 1,
			steps: 0,
			floor: isize::max_value(),
		};

		if floor {
			simulation.floor = max.1 + 2;
			simulation.max.1 += 2;
		}

		simulation
	}

	fn step(&mut self) -> bool {
		let mut complete: bool = false;

		let floor = self.floor != isize::max_value() && self.sand.1 >= self.floor - 1;

		if !floor && !self.grid.contains_key(&(self.sand.0, self.sand.1 + 1)) {
			self.sand.1 += 1;
		} else if !floor && !self.grid.contains_key(&(self.sand.0 - 1, self.sand.1 + 1)) {
			self.sand.0 -= 1;
			self.sand.1 += 1;
		} else if !floor && !self.grid.contains_key(&(self.sand.0 + 1, self.sand.1 + 1)) {
			self.sand.0 += 1;
			self.sand.1 += 1;
		} else {
			self.grid.insert((self.sand.0, self.sand.1), Obstacle::SAND);

			if self.sand.0 == 500 && self.sand.1 == 0 {
				complete = true;
			}

			self.sand = (500, 0);
			self.sand_units += 1;
		}

		if self.sand.1 > self.max.1 {
			complete = true;
		}

		self.min.0 = if self.sand.0 < self.min.0 { self.sand.0 } else { self.min.0 };
		self.min.1 = if self.sand.1 < self.min.1 { self.sand.1 } else { self.min.1 };
		self.max.0 = if self.sand.0 > self.max.0 { self.sand.0 } else { self.max.0 };
		self.max.1 = if self.sand.1 > self.max.1 { self.sand.1 } else { self.max.1 };

		self.steps += 1;

		complete
	}

	fn get_num_sand_units(&mut self, draw: bool) -> usize {
		loop {
			let complete = self.step();

			if draw {
				self._print();
				thread::sleep(Duration::from_millis(8)); // ~ 120 FPS :p
			}

			if complete {
				break;
			}

		}

		self.sand_units - 1
	}

	fn _print(&self) {
		for y in self.min.1 .. self.max.1 + 1 {
			let mut output = "".to_string();

			for x in self.min.0 .. self.max.0 + 1 {
				if self.grid.contains_key(&(x, y)) {
					output.push_str(self.grid.get(&(x, y)).unwrap().to_string().as_str());
				} else if self.sand.0 == x && self.sand.1 == y {
					output.push_str("o");
				} else if self.floor != isize::max_value() && y == self.max.1 {
					output.push_str("#");
				} else {
					output.push_str(".");
				}
			}

			println!("{}", output);
		}
	}
}

pub fn part1(input: String) {
	let mut simulation: Simulation = Simulation::new(input, false);
	
	println!("{}", simulation.get_num_sand_units(false));
}

pub fn part2(input: String) {
	let mut simulation: Simulation = Simulation::new(input, true);
	
	println!("{}", simulation.get_num_sand_units(false));
}
