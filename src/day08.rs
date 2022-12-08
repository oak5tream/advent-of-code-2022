fn solve(input: String) -> (usize, usize) {
	let trees: Vec<Vec<usize>> = input
		.lines()
		.map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
		.collect();

	let width: usize = trees.len();
	let height: usize = trees[0].len();

	let mut num_visible: usize = width * 2 + height * 2 - 4;
	let mut max_scenic_score: usize = 0;

	for y in 1 .. height - 1 {
		for x in 1 .. width - 1 {
			fn scan_horizontal(trees: &Vec<Vec<usize>>, x: usize, y: usize, backwards: bool) -> (bool, usize) {
				let mut scenic_score: usize = 0;
				let mut x_cmp: isize = if backwards { x as isize - 1 } else { x as isize + 1 };

				while x_cmp >= 0 && x_cmp < trees.len() as isize {
					scenic_score += 1;

					if trees[y][x] <= trees[y][x_cmp as usize] {
						return (false, scenic_score);
					}

					x_cmp += if backwards { -1 } else { 1 };
				}

				(true, scenic_score)
			}

			fn scan_vertical(trees: &Vec<Vec<usize>>, x: usize, y: usize, backwards: bool) -> (bool, usize) {
				let mut scenic_score: usize = 0;
				let mut y_cmp: isize = if backwards { y as isize - 1 } else { y as isize + 1 };

				while y_cmp >= 0 && y_cmp < trees[0].len() as isize {
					scenic_score += 1;

					if trees[y][x] <= trees[y_cmp as usize][x] {
						return (false, scenic_score);
					}

					y_cmp += if backwards { -1 } else { 1 };
				}

				(true, scenic_score)
			}

			let mut visibility: Vec<bool> = vec![false; 4];
			let mut score: Vec<usize> = vec![0; 4];

			(visibility[0], score[0]) = scan_horizontal(&trees, x, y, false);
			(visibility[1], score[1]) = scan_horizontal(&trees, x, y, true);
			(visibility[2], score[2]) = scan_vertical(&trees, x, y, false);
			(visibility[3], score[3]) = scan_vertical(&trees, x, y, true);

			if visibility[0] | visibility[1] | visibility[2] |visibility[3] {
				num_visible += 1;
			}

			let scenic_score: usize = score[0] * score[1] * score[2] * score[3];
			
			if scenic_score > max_scenic_score {
				max_scenic_score = scenic_score;
			}
		}
	}

	(num_visible, max_scenic_score)
}

pub fn part1(input: String) {
	let (num_visible, _) = solve(input);

	println!("Num visible: {}", num_visible);
}

pub fn part2(input: String) {
	let (_, scenic_score) = solve(input);
	
	println!("Scenic scores: {}", scenic_score);
}
