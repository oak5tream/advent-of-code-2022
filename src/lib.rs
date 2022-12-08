pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

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
		_ => {
			println!("Unknown day: {}", day);
			return (noop, noop);
		}
	};
}

