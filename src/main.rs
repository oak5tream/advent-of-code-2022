// Template inspiration: https://github.com/nickyvanurk/advent-of-code-rust-template

use std::env;
use std::fs;
use std::time::{Duration, Instant};

use advent_of_code::{get_day, noop};

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_duration(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

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
		let start = Instant::now();
		to_run.0(input.clone());
		let duration = start.elapsed();
		println!("Duration: {}", fmt_duration(duration));
	}

	if to_run.1 != noop {
		let start = Instant::now();
		to_run.1(input.clone());
		let duration = start.elapsed();
		println!("Duration: {}", fmt_duration(duration));
	}
}

