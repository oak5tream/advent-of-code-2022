use std::collections::{HashMap, HashSet};
use std::fmt;

use itertools::Itertools;

struct Shape;

impl Shape {
	pub const MINUS: u8 = 0;
	pub const PLUS: u8 = 1;
	pub const J: u8 = 2;
	pub const I: u8 = 3;
	pub const SQUARE: u8 = 4;

}

struct Rock {
	shape: u8,
	pos: (isize, isize),
}

impl Rock {
	fn new(shape: u8, pos: (isize, isize)) -> Rock {
		Rock {
			shape,
			pos,
		}
	}

	fn get_coords(&self) -> Vec<(isize, isize)> {
		let mut coords: Vec<(isize, isize)> = vec![];

		match self.shape {
			Shape::MINUS => {
				coords.push((self.pos.0 + 0, self.pos.1));
				coords.push((self.pos.0 + 1, self.pos.1));
				coords.push((self.pos.0 + 2, self.pos.1));
				coords.push((self.pos.0 + 3, self.pos.1));
			},

			Shape::PLUS => {
				coords.push((self.pos.0 + 1, self.pos.1 - 2));
				coords.push((self.pos.0 + 0, self.pos.1 - 1));
				coords.push((self.pos.0 + 1, self.pos.1 - 1));
				coords.push((self.pos.0 + 2, self.pos.1 - 1));
				coords.push((self.pos.0 + 1, self.pos.1 - 0));
			},

			Shape::J => {
				coords.push((self.pos.0 + 2, self.pos.1 - 2));
				coords.push((self.pos.0 + 2, self.pos.1 - 1));
				coords.push((self.pos.0 + 0, self.pos.1 - 0));
				coords.push((self.pos.0 + 1, self.pos.1 - 0));
				coords.push((self.pos.0 + 2, self.pos.1 - 0));
			},

			Shape::I => {
				coords.push((self.pos.0, self.pos.1 - 3));
				coords.push((self.pos.0, self.pos.1 - 2));
				coords.push((self.pos.0, self.pos.1 - 1));
				coords.push((self.pos.0, self.pos.1 - 0));
			},

			Shape::SQUARE => {
				coords.push((self.pos.0 + 0, self.pos.1 - 1));
				coords.push((self.pos.0 + 1, self.pos.1 - 1));
				coords.push((self.pos.0 + 0, self.pos.1 - 0));
				coords.push((self.pos.0 + 1, self.pos.1 - 0));
			},

			_ => panic!(),
		}

		coords
	}
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct BoardHash {
	lines: Vec<u8>,
	pattern_index: usize,
	shape: u8,
}

struct Board {
	min: (isize, isize),
	max: (isize, isize),
	grid: HashSet<(isize, isize)>,
	rock: Rock,
	rocks_stopped: usize,
	pattern: Vec<char>,
	pattern_index: usize,
	hashes: HashMap<BoardHash, (usize, usize)>,
}

impl Board {
	fn new(mut pattern: Vec<char>) -> Board {
		pattern.pop();

		Board {
			min: (0, -3),
			max: (6, 0),
			grid: HashSet::new(),
			rock: Rock::new(Shape::MINUS, (2, -3)),
			rocks_stopped: 0,
			pattern,
			pattern_index: 0,
			hashes: HashMap::new(),
		}
	}

	fn get_height_after_num_rocks(&mut self, num_rocks: usize) -> usize {
		let mut added_height: usize = 0;

		while self.rocks_stopped < num_rocks {
			let x_offset: isize = if self.pattern[self.pattern_index] == '<' { -1 } else { 1 };

			fn can_move(grid: &HashSet<(isize, isize)>, coords: &Vec<(isize, isize)>, offset: (isize, isize)) -> bool {
				for coord in coords {
					if coord.0 + offset.0 < 0 || coord.0 + offset.0 > 6 || coord.1 + offset.1 > 0 || 
						grid.contains(&(coord.0 + offset.0, coord.1 + offset.1)) {
							return false;
					}
				}

				true
			}

			if can_move(&self.grid, &self.rock.get_coords(), (x_offset, 0)) {
				self.rock.pos.0 += x_offset;
			}

			if can_move(&self.grid, &self.rock.get_coords(), (0, 1)) {
				self.rock.pos.1 += 1;
			} else {
				for coord in &self.rock.get_coords() {
					self.grid.insert((coord.0, coord.1));
				}

				let mut min_y: isize = 0;
				for grid_coord in &self.grid {
					min_y = min_y.min(grid_coord.1);
				}
				
				self.rocks_stopped += 1;
				self.rock.shape = (self.rock.shape + 1) % 5;
				self.rock.pos = (2, min_y - 4);

				for coord in &self.rock.get_coords() {
					self.min.1 = self.min.1.min(coord.1);
				}

				let hash: BoardHash = self.calculate_hash(min_y);

				if min_y < 100 && added_height == 0 {
					if self.hashes.contains_key(&hash) {
						let current_height: usize = self.get_height();
						let (cached_height, cached_rocks) = self.hashes.get(&hash).unwrap();
						let diff_height: usize = current_height - cached_height;
						let diff_rocks: usize = self.rocks_stopped - cached_rocks;
						let add_blocks: usize = (num_rocks - self.rocks_stopped) / diff_rocks;
						let add_height: usize = add_blocks * diff_height;

						added_height = add_height;
						self.rocks_stopped += add_blocks * diff_rocks;
					} else {
						self.hashes.insert(hash, (self.get_height(), self.rocks_stopped));
					}
				}
			}

			self.pattern_index = (self.pattern_index + 1) % self.pattern.len();
		}

		self.get_height() + added_height
	}

	fn get_height(&self) -> usize {
		let mut min_y: isize = 0;

		for coord in &self.grid {
			min_y = min_y.min(coord.1);
		}

		(min_y * -1 + 1) as usize
	}

	fn calculate_hash(&self, start_y: isize) -> BoardHash {
		let mut hash: BoardHash = BoardHash {
			lines: vec![],
			pattern_index: self.pattern_index,
			shape: self.rock.shape,
		};

		for y in start_y .. start_y + 20 {
			let mut line: u8  = 0;

			for x in 0 .. 7 {
				if self.grid.contains(&(x, y)) {
					line += 1 << x;
				}
			}

			hash.lines.push(line);
		}

		hash
	}
}
	
impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let rock_coords: Vec<(isize, isize)> = self.rock.get_coords();
		let mut output: String = "".to_string();

		for y in self.min.1 .. self.max.1 + 1 {
			for x in self.min.0 .. self.max.0 + 1 {
				if rock_coords.contains(&(x, y)) {
					output.push_str("@");
				} else if self.grid.contains(&(x, y)) {
					output.push_str("#");
				} else {
					output.push_str(".");
				}
			}

			output.push_str("\n");
		}

		write!(f, "{}", output)
    }
}

pub fn part1(input: String) {
	let mut board: Board = Board::new(input.chars().collect_vec());

	println!("{}", board.get_height_after_num_rocks(2022));
}

pub fn part2(input: String) {
	let mut board: Board = Board::new(input.chars().collect_vec());

	println!("{}", board.get_height_after_num_rocks(1000000000000));
}
