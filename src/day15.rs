use regex::Regex;

struct Sensor {
	pos: (isize, isize),
	beacon: (isize, isize),
	radius: isize,
}

impl Sensor {
	fn new(pos: (isize, isize), beacon: (isize, isize)) -> Sensor {
		Sensor {
			pos,
			beacon,
			radius: (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs(),
		}
	}

	fn is_visible(&self, pos: (isize, isize)) -> bool {
		(self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs() <= self.radius
	}

	fn get_min_bounds(&self) -> (isize, isize) {
		(self.pos.0 - self.radius, self.pos.1 - self.radius)
	}

	fn get_max_bounds(&self) -> (isize, isize) {
		(self.pos.0 + self.radius, self.pos.1 + self.radius)
	}
}

fn parse(input: String) -> Vec<Sensor> {
	let mut sensors: Vec<Sensor> = vec![];

	for line in input.lines() {
		let re = Regex::new(r"r at x=([-0-9]+), y=([-0-9]+).*x=([-0-9]+), y=([-0-9]+)").unwrap();

		for captures in re.captures_iter(line) {
			let s_x: isize = captures[1].parse::<isize>().unwrap();
			let s_y: isize = captures[2].parse::<isize>().unwrap();
			let b_x: isize = captures[3].parse::<isize>().unwrap();
			let b_y: isize = captures[4].parse::<isize>().unwrap();

			sensors.push(Sensor::new((s_x, s_y), (b_x, b_y)));
		}
	}

	sensors
}

pub fn part1(input: String) {
	let sensors = parse(input);
	let mut min: (isize, isize) = (isize::max_value(), isize::max_value());
	let mut max: (isize, isize) = (isize::min_value(), isize::min_value());

	for sensor in &sensors {
		let min_bounds = sensor.get_min_bounds();
		let max_bounds = sensor.get_max_bounds();

		min.0 = if min_bounds.0 < min.0 { min_bounds.0 } else { min.0 };
		min.1 = if min_bounds.1 < min.1 { min_bounds.1 } else { min.1 };
		max.0 = if max_bounds.0 > max.0 { max_bounds.0 } else { max.0 };
		max.1 = if max_bounds.1 > max.1 { max_bounds.1 } else { max.1 };

	}

	let mut num_empty_positions: usize = 0;
//	let y = 10;			// Test case
	let y = 2000000;	// Real case

	for x in min.0 .. max.0 + 1 {
		let mut visible: bool = false;
		let mut scanner_at_pos: bool = false;

		for sensor in &sensors {
			visible |= sensor.is_visible((x, y));
			scanner_at_pos |= sensor.beacon.0 == x && sensor.beacon.1 == y;
		}

		if visible && !scanner_at_pos {
			num_empty_positions += 1;
		}
	}

	println!("{}", num_empty_positions);
}

pub fn part2(input: String) {
	let sensors = parse(input);

	let mut matching_sensors: Vec<usize> = vec![];
	let mut coords: Vec<(isize, isize)> = vec![];

	for j in 0 .. sensors.len() {
		for i in 0 .. sensors.len() {
			if i != j {
				let pos0 = sensors[i].pos;
				let pos1 = sensors[j].pos;
				let distance = (pos0.0 - pos1.0).abs() + (pos0.1 - pos1.1).abs();

				if distance == sensors[i].radius + sensors[j].radius + 2 {
					if !matching_sensors.contains(&i) {
						matching_sensors.push(i);
					}

					if !matching_sensors.contains(&j) {
						matching_sensors.push(j);
					}
				}
			}
		}
	}

	for i in 0 .. matching_sensors.len() {
		let sensor = &sensors[i];

		let mut coord: (isize, isize) = (sensor.pos.0, sensor.pos.1 - sensor.radius - 1);
		
		for _ in 0 .. sensor.radius + 2 {
			coord.0 += 1;
			coord.1 += 1;
			coords.push(coord.clone());
		}

		for _ in 0 .. sensor.radius + 2 {
			coord.0 -= 1;
			coord.1 += 1;
			coords.push(coord.clone());
		}

		for _ in 0 .. sensor.radius + 2 {
			coord.0 -= 1;
			coord.1 -= 1;
			coords.push(coord.clone());
		}

		for _ in 0 .. sensor.radius + 2 {
			coord.0 += 1;
			coord.1 -= 1;
			coords.push(coord.clone());
		}
	}

	for coord in coords {
		if coord.0 < 0 || coord.1 < 0 || coord.0 > 4000000 || coord.1 > 4000000 {
			continue;
		}

		let mut visible: bool = false;

		for sensor in &sensors {
			visible |= sensor.is_visible(coord);
		}

		if !visible {
			println!("{}", coord.0 * 4000000 + coord.1);
		}
	}
}
