use std::collections::HashSet;
use std::iter::FromIterator;

fn get_start_index(input: &str, num_chars: usize) -> usize {
	let buffer: Vec<char> = input.chars().collect();

	for (index, window) in buffer.windows(num_chars).enumerate() {
		let hash: HashSet<char> = HashSet::from_iter(window.iter().cloned());

		if hash.len() == num_chars {
			return index + num_chars;
		}
	}

	return 0;
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
