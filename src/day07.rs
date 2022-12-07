use regex::Regex;
use std::collections::HashMap;

struct Node {
	name: String,
	is_dir: bool,
}

impl Node {
	fn new(name: &str, is_dir: bool) -> Node {
		Node {
			name: String::from(name),
			is_dir,
		}
	}
}

struct Tree {
	nodes: HashMap<usize, Node>,
	parents: HashMap<usize, usize>,
	children: HashMap<usize, Vec<usize>>,
	sizes: HashMap<usize, usize>,
	current_id: usize,
}

impl Tree {
	fn new() -> Tree {
		Tree {
			nodes: HashMap::new(),
			parents: HashMap::new(),
			children: HashMap::new(),
			sizes: HashMap::new(),
			current_id: 0,
		}
	}

	fn add_node(&mut self, id: usize, name: &str, size: usize, is_dir: bool) {
		let node: Node = Node::new(name, is_dir);

		self.nodes.insert(id, node);
		self.parents.insert(id, self.current_id);

		if id != self.current_id {
			self.children.entry(self.current_id).or_insert(vec![]).push(id);
		}

		self.sizes.insert(id, size);
		self.accumulate_sizes(id, size);
	}

	fn accumulate_sizes(&mut self, id: usize, size: usize) {
		let parent_id = self.parents[&id];

		*self.sizes.entry(parent_id).or_insert(size) += size;

		if id != 0 {
			self.accumulate_sizes(parent_id, size);
		}
	}

	fn goto_parent(&mut self) {
		self.current_id = self.parents[&self.current_id];
	}

	fn goto_child(&mut self, name: &str) {
		for child in &self.children[&self.current_id] {
			let child_node: &Node = &self.nodes[child];

			if child_node.name == name {
				self.current_id = *child;
				break;
			}
		}
	}

	fn get_dirs_with_max_size(&self, max_size: usize) -> Vec<usize> {
		let mut result: Vec<usize> = vec![];

		for (id, size) in &self.sizes {
			let node: &Node = &self.nodes[&id];

			if node.is_dir && size <= &max_size {
				result.push(*size);
			}
		}

		result
	}

	fn get_total_size(&self) -> usize {
		let mut result: usize = 0;

		for (id, size) in &self.sizes {
			let node: &Node = &self.nodes[&id];

			if !node.is_dir {
				result += *size;
			}
		}

		result
	}

	// TODO: Change to format!
	fn _print(&self, id: usize, level: usize) {
		let mut output = "".to_string();
		
		for _ in 0 .. level * 2 {
			output.push_str("  ");
		}

		let node: &Node = &self.nodes[&id];
		let size: usize = self.sizes[&id];

		if node.is_dir {
			output.push_str(&format!("{} (dir, size: {})", node.name, size));
		} else {
			output.push_str(&format!("{} (file, size: {})", node.name, size));
		}

		println!("{}", output);

		if self.children.contains_key(&id) {
			for &child in &self.children[&id] {
				self._print(child, level + 1);
			}
		}
	}
}

fn populate_tree(input: String) -> Tree {
	let mut tree: Tree = Tree::new();

	tree.add_node(0, "/", 0, true);

	for (id, line) in input.lines().enumerate() {
		if line.starts_with("$ cd") {
			let re = Regex::new(r"([a-z]+) (.+)").unwrap();
			let caps = re.captures(line).unwrap();
			let dir = caps.get(2).unwrap().as_str();

			if dir == ".." {
				tree.goto_parent();
			} else if dir != "/" {
				tree.goto_child(dir);
			}
		} else if !line.starts_with("$ ls") {
			if line.starts_with("dir") {
				let re = Regex::new(r".+ (.+)").unwrap();
				let caps = re.captures(line).unwrap();
				let dir = caps.get(1).unwrap().as_str();

				tree.add_node(id, dir, 0, true);
			} else {
				let re = Regex::new(r"([0-9]+) (.+)").unwrap();
				let caps = re.captures(line).unwrap();
				let size = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
				let filename = caps.get(2).unwrap().as_str();

				tree.add_node(id, filename, size, false);
			}
		}
	}

	tree
}

pub fn part1(input: String) {
	let tree: Tree = populate_tree(input);

	let mut result: usize = 0;

	for size in tree.get_dirs_with_max_size(100000) {
		result += size;
	}

	println!("{}", result);
}

pub fn part2(input: String) {
	let tree: Tree = populate_tree(input);
	let max_space: usize = 70000000;
	let min_space: usize = 30000000;
	let fs_space: usize = tree.get_total_size();
	let to_free: usize = min_space - (max_space - fs_space);

	let mut sizes: Vec<usize> = vec![];

	for (id, size) in &tree.sizes {
		let node: &Node = &tree.nodes[&id];

		if node.is_dir {
			sizes.push(*size);
		}
	}

	sizes.sort();
	sizes.reverse();

	let mut smallest: usize = usize::max_value();

	for size in &sizes {
		if size < &to_free {
			break;
		}

		smallest = *size;
	}

	println!("{}", smallest);
}
