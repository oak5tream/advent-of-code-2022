use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
struct Valve {
	id: usize,
	name: String,
	flow: usize,
	connections: Vec<usize>,
}

impl Valve {
	fn new(id: usize, name: String, flow: usize) -> Valve {
		Valve {
			id,
			name,
			flow,
			connections: vec![],
		}
	}

	fn add_connection(&mut self, id: usize) {
		if !self.connections.contains(&id) {
			self.connections.push(id);
		}
	}

	fn _print(&self) {
		println!("Valve {} ({}), flow: {}, connections: {:?}",
		self.name, self.id, self.flow, self.connections);
	}
}

fn parse(input: String) -> Vec<Valve> {
	let mut valves: Vec<Valve> = vec![];
	let mut connections: Vec<Vec<String>> = vec![];

	for (id, line) in input.lines().enumerate() {
		let re = Regex::new(r"^Valve ([A-Z]+).*rate=([0-9]+).*valve[s]* (.*)$").unwrap();
		for captures in re.captures_iter(line) {
			let name = captures[1].to_string();
			let rate = captures[2].parse::<usize>().unwrap();
			let cons = captures[3].to_string();
			
			connections.push(vec![]);

			if cons.contains(",") {
				for con in cons.split(", ") {
					connections[id].push(con.to_string());
				}
			} else {
				connections[id].push(cons);
			}

			valves.push(Valve::new(id, name, rate));
		}
	}

	for id in 0 .. connections.len() {
		let connection = &connections[id];
	
		for con in connection {
			let mut valve_id: isize = -1;

			for valve in &valves {
				if valve.name == con.clone() {
					valve_id = valve.id as isize;
				}
			}

			if valve_id > -1 {
				valves[id].add_connection(valve_id as usize);
			}
		}
	}

	valves
}

fn get_max_flow(id: usize, valves: &Vec<Valve>, forbidden_valves: &Vec<usize>,
				path: &mut Vec<usize>, time_left: isize,
				visited: &mut HashMap<(usize, isize, Vec<usize>), isize>) -> isize {

	if time_left <= 0 || forbidden_valves.contains(&id) {
		return 0;
	}
	
	if visited.contains_key(&(id, time_left, path.clone())) {
		return *visited.get(&(id, time_left, path.clone())).unwrap();
	}

	let mut max_flow: isize = isize::min_value();
	let flow: isize = valves[id].flow as isize;

	if flow > 0 && !path.contains(&id) {
		for connection in &valves[id].connections {
			if forbidden_valves.contains(connection) {
				continue;
			}

			path.push(id);

			let branch_flow = get_max_flow(*connection, valves, forbidden_valves, path, time_left - 2, visited);

			max_flow = max_flow.max(branch_flow + flow * (time_left - 1));
			path.pop();
		}
	}

	for connection in &valves[id].connections {
		let branch_flow = get_max_flow(*connection, valves, forbidden_valves, path, time_left - 1, visited);
		max_flow = max_flow.max(branch_flow);
	}
	
	visited.insert((id, time_left, path.clone()), max_flow);

	max_flow
}

fn get_start_id(valves: &Vec<Valve>) -> usize {
	for valve in valves {
		if valve.name == "AA" {
			return valve.id;
		}
	}

	return 0;
}

fn solve_alone(valves: &Vec<Valve>) -> isize {
	let start_id: usize = get_start_id(valves);
	let mut visited: HashMap<(usize, isize, Vec<usize>), isize> = HashMap::new();
	let mut path: Vec<usize> = vec![];

	get_max_flow(start_id, &valves, &vec![], &mut path, 30, &mut visited)
}

fn solve_together(valves: &Vec<Valve>) -> isize {
	let start_id: usize = get_start_id(valves);

	let mut lists: Vec<Vec<usize>> = vec![];
	let mut valve_ids: Vec<usize> = vec![];

	for valve in valves {
		if valve.flow > 0 {
			valve_ids.push(valve.id);
		}
	}

	let mut list_size: usize = valve_ids.len() / 2;

	if valve_ids.len() % 2 == 1 {
		list_size += 1;
	}

	for perm in valve_ids.into_iter().combinations(list_size) {
		lists.push(perm.clone())
	}

	let mut max_flow: isize = 0;

	for (index, forbidden_0) in lists.iter().enumerate() {
		if index % 10 == 0 {
			println!("Calculating {} / {}...", index, lists.len());
		}

		let mut forbidden_1: Vec<usize> = vec![];

		for i in 0 .. valves.len() {
			if !forbidden_0.contains(&i) && valves[i].flow > 0 {
				forbidden_1.push(i);
			}
		}

		let mut visited: HashMap<(usize, isize, Vec<usize>), isize> = HashMap::new();
		let mut path: Vec<usize> = vec![];

		let flow_0: isize = get_max_flow(start_id, &valves, &forbidden_0, &mut path, 26, &mut visited);

		visited = HashMap::new();
		path = vec![];

		let flow_1: isize = get_max_flow(start_id, &valves, &forbidden_1, &mut path, 26, &mut visited);

		max_flow = max_flow.max(flow_0 + flow_1);
	}

	return max_flow;
}

pub fn part1(input: String) {
	let valves: Vec<Valve> = parse(input);

	println!("{}", solve_alone(&valves));
}

pub fn part2(input: String) {
	let valves: Vec<Valve> = parse(input);

	println!("{}", solve_together(&valves));
}
