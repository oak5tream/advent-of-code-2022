fn get_sorted_calories(input: String) -> Vec<usize> {
	let mut sum: usize = 0;
	let mut calories: Vec<usize> = vec![];

	for line in input.lines() {
		if line == "" {
			calories.push(sum);
			sum = 0;
		} else {
			sum += line.parse::<usize>().unwrap();
		}
	}

	calories.push(sum);
	calories.sort();

	calories
}

pub fn part1(input: String) {
	let calories: Vec<usize> = get_sorted_calories(input);	

	println!("{}", calories.last().unwrap());
}

pub fn part2(input: String) {
	let calories: Vec<usize> = get_sorted_calories(input);	
	let result: usize = calories.iter().rev().take(3).sum();

	println!("{}", result);
}
