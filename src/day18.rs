use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Cube(isize, isize, isize);

fn parse(input: String) -> HashSet<Cube> {
	let mut cubes: HashSet<Cube> = HashSet::new();

	for line in input.lines() {
		let mut coords = line.split(",");
		let x = coords.next().unwrap().parse::<isize>().unwrap();
		let y = coords.next().unwrap().parse::<isize>().unwrap();
		let z = coords.next().unwrap().parse::<isize>().unwrap();

		cubes.insert(Cube(x, y, z));
	}

	cubes
}

pub fn part1(input: String) {
	let cubes = parse(input);
	let mut surface_area: usize = 0;

	for c in &cubes {
		surface_area += if !cubes.contains(&Cube(c.0 - 1, c.1, c.2)) { 1 } else { 0 };
		surface_area += if !cubes.contains(&Cube(c.0 + 1, c.1, c.2)) { 1 } else { 0 };
		surface_area += if !cubes.contains(&Cube(c.0, c.1 - 1, c.2)) { 1 } else { 0 };
		surface_area += if !cubes.contains(&Cube(c.0, c.1 + 1, c.2)) { 1 } else { 0 };
		surface_area += if !cubes.contains(&Cube(c.0, c.1, c.2 - 1)) { 1 } else { 0 };
		surface_area += if !cubes.contains(&Cube(c.0, c.1, c.2 + 1)) { 1 } else { 0 };
	}

	println!("{}", surface_area);
}

pub fn part2(input: String) {
	let cubes = parse(input);
	let mut min: Cube = Cube(isize::max_value(), isize::max_value(), isize::max_value());
	let mut max: Cube = Cube(isize::min_value(), isize::min_value(), isize::min_value());

	for c in &cubes {
		min.0 = if c.0 < min.0 { c.0 } else { min.0 };
		min.1 = if c.1 < min.1 { c.1 } else { min.1 };
		min.2 = if c.2 < min.2 { c.2 } else { min.2 };
		max.0 = if c.0 > max.0 { c.0 } else { max.0 };
		max.1 = if c.1 > max.1 { c.1 } else { max.1 };
		max.2 = if c.2 > max.2 { c.2 } else { max.2 };
	}

	fn can_reach_bounds(cube: Cube, cubes: &HashSet<Cube>, min: &Cube, max: &Cube, visited: &mut HashSet<Cube>) -> bool {
		let mut reached: bool = false;

		if cubes.contains(&cube) || visited.contains(&cube) {
			return false;
		}

		if cube.0 < min.0 || cube.1 < min.1 || cube.2 < min.2 ||
			cube.0 > max.0 || cube.1 > max.1 || cube.2 > max.2 {
				return true;
			}

		visited.insert(cube);

		reached |= can_reach_bounds(Cube(cube.0 - 1, cube.1, cube.2), cubes, min, max, visited);
		reached |= can_reach_bounds(Cube(cube.0 + 1, cube.1, cube.2), cubes, min, max, visited);
		reached |= can_reach_bounds(Cube(cube.0, cube.1 - 1, cube.2), cubes, min, max, visited);
		reached |= can_reach_bounds(Cube(cube.0, cube.1 + 1, cube.2), cubes, min, max, visited);
		reached |= can_reach_bounds(Cube(cube.0, cube.1, cube.2 - 1), cubes, min, max, visited);
		reached |= can_reach_bounds(Cube(cube.0, cube.1, cube.2 + 1), cubes, min, max, visited);

		reached
	}

	let mut surface_area: usize = 0;

	for c in &cubes {
		let mut visited: HashSet<Cube> = HashSet::new();
		surface_area += if can_reach_bounds(Cube(c.0 - 1, c.1, c.2), &cubes, &min, &max, &mut visited) { 1 } else { 0 };

		visited.clear();
		surface_area += if can_reach_bounds(Cube(c.0 + 1, c.1, c.2), &cubes, &min, &max, &mut visited) { 1 } else { 0 };

		visited.clear();
		surface_area += if can_reach_bounds(Cube(c.0, c.1 - 1, c.2), &cubes, &min, &max, &mut visited) { 1 } else { 0 };

		visited.clear();
		surface_area += if can_reach_bounds(Cube(c.0, c.1 + 1, c.2), &cubes, &min, &max, &mut visited) { 1 } else { 0 };

		visited.clear();
		surface_area += if can_reach_bounds(Cube(c.0, c.1, c.2 - 1), &cubes, &min, &max, &mut visited) { 1 } else { 0 };

		visited.clear();
		surface_area += if can_reach_bounds(Cube(c.0, c.1, c.2 + 1), &cubes, &min, &max, &mut visited) { 1 } else { 0 };
	}

	println!("{}", surface_area);
}
