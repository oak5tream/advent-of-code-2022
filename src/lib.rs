pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
	return match day {
		01 => (day01::part1, day01::part2),
		02 => (day02::part1, day02::part2),
		03 => (day03::part1, day03::part2),
		04 => (day04::part1, day04::part2),
		05 => (day05::part1, day05::part2),
		06 => (day06::part1, day06::part2),
		07 => (day07::part1, day07::part2),
		08 => (day08::part1, day08::part2),
		09 => (day09::part1, day09::part2),
		10 => (day10::part1, day10::part2),
		11 => (day11::part1, day11::part2),
		12 => (day12::part1, day12::part2),
		13 => (day13::part1, day13::part2),
		14 => (day14::part1, day14::part2),
		15 => (day15::part1, day15::part2),
		16 => (day16::part1, day16::part2),
		17 => (day17::part1, day17::part2),
		18 => (day18::part1, day18::part2),
		19 => (day19::part1, day19::part2),
		_ => {
			println!("Unknown day: {}", day);
			return (noop, noop);
		}
	};
}

