// Template inspiration: https://github.com/nickyvanurk/advent-of-code-rust-template

use std::env;
use std::fs;

use advent_of_code::{get_day, noop};

fn main() {
	let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
		println!("Expected a day and optional test");
        return;
    }

	let mut day = args[1].clone();
	let mut test = "".to_string();

    if args.len() == 3 {
		test.push_str(&format!("_{}", args[2]));
	}

	day = day.trim().to_string();
	let day_num: u32 = match day.parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Invalid day number: {}", day);
			return;
		}
	};

	let cwd = env::current_dir().unwrap();
	let filename = cwd.join("data").join(format!("day{:02}{}.txt", day_num, test));
	println!("Reading {}", filename.display());
	let input = fs::read_to_string(filename).expect("Error while reading");
	let to_run = get_day(day_num);

	if to_run.0 != noop {
		to_run.0(input.clone());
	}

	if to_run.1 != noop {
		to_run.1(input.clone());
	}
}

