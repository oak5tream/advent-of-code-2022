use std::collections::HashSet;
use std::iter::FromIterator;

fn get_start_index(input: &str, num_chars: usize) -> usize {
	let buffer: Vec<char> = input.chars().collect();

	let mut index: usize = num_chars;

	for window in buffer.windows(num_chars) {
		let hash: HashSet<char> = HashSet::from_iter(window.iter().cloned());
		if hash.len() == num_chars {
			break;
		}
			
		index += 1;
	}

	index
}

pub fn part1(input: String) {
	for line in input.lines() {
		println!("{}", get_start_index(line, 4));
	}
}

pub fn part2(input: String) {
	for line in input.lines() {
		println!("{}", get_start_index(line, 14));
	}
}
