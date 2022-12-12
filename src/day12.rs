use fast_paths::InputGraph;

fn find_min_steps(elevation: Vec<Vec<usize>>, start_id: usize, end_id: usize) -> usize {
	let mut input_graph = InputGraph::new();	

	let height = elevation.len();
	let width = elevation[0].len();

	for y in 0 .. height {
		for x in 0 .. width {
			let id: usize = y * width + x;

			let val: usize = elevation[y][x];

			if x > 0 {
				if elevation[y][x - 1] <= val + 1 {
					let left_id = y * width + x - 1;
					input_graph.add_edge(id, left_id, 1);
				}
			}

			if x < width - 1 {
				if elevation[y][x + 1] <= val + 1 {
					let right_id = y * width + x + 1;
					input_graph.add_edge(id, right_id, 1);
				}
			}

			if y > 0 {
				if elevation[y - 1][x] <= val + 1 {
					let up_id = (y - 1) * width + x;
					input_graph.add_edge(id, up_id, 1);
				}
			}

			if y < height - 1 {
				if elevation[y + 1][x] <= val + 1 {
					let down_id = (y + 1) * width + x;
					input_graph.add_edge(id, down_id, 1);
				}
			}
		}
	}

	input_graph.freeze();

	let fast_graph = fast_paths::prepare(&input_graph);
	let shortest_path = fast_paths::calc_path(&fast_graph, start_id, end_id);

	match shortest_path {
	    Some(p) => {
			return p.get_weight() as usize;
	    },
	    None => {
			println!("Couldn't find path");
			return 0 as usize;
	    }
	}
}

fn parse(input: String) -> (Vec<Vec<usize>>, usize, usize) {
	let mut start_id: usize = 0;
	let mut end_id: usize = 0;

	let mut elevation: Vec<Vec<usize>> = input
		.lines()
		.map(|s| s.chars().map(|c| if c == 'S' {
			100 as usize
		} else if c == 'E' {
			200 as usize
		} else {
			c as usize - 97
		}).collect())
		.collect();

	for y in 0 .. elevation.len() {
		for x in 0 .. elevation[0].len() {
			let id: usize = y * elevation[0].len() + x;

			if elevation[y][x] == 100 {
				elevation[y][x] = 0;
				start_id = id;
			} else if elevation[y][x] == 200 {
				elevation[y][x] = 25;
				end_id = id;
			}
		}
	}

	(elevation, start_id, end_id)
}

pub fn part1(input: String) {
	let (elevation, start_id, end_id) = parse(input);
	let steps: usize = find_min_steps(elevation, start_id, end_id);
	println!("{}", steps);
}

pub fn part2(input: String) {
	let (elevation, _, end_id) = parse(input);
	let mut start_ids: Vec<usize> = vec![];

	for y in 0 .. elevation.len() {
		for x in 0 .. elevation[0].len() {
			if elevation[y][x] == 0 {
				start_ids.push(y * elevation[0].len() + x);
			}
		}
	}

	let mut steps: Vec<usize> = vec![];

	let mut index = 0;
	for start_id in &start_ids {
		index += 1;
		println!("Testing start_id {} ({} / {})", start_id, index, &start_ids.len());
		let step = find_min_steps(elevation.clone(), *start_id, end_id);

		if step > 0 {
			steps.push(step);
		}
	}

	steps.sort();
	println!("{}", steps[0]);
}
