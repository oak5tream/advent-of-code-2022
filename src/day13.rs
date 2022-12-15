use itertools::Itertools;
use std::collections::HashMap;

struct Node {
	value: usize,
	is_list: bool,
	
}

impl Node {
	fn new(value: usize, is_list: bool) -> Node {
		Node {
			value,
			is_list,
		}
	}
}

struct Tree {
	nodes: HashMap<usize, Node>,
	parents: HashMap<usize, usize>,
	children: HashMap<usize, Vec<usize>>,
	current_id: usize,
	max_id: usize,
}

// Rename to SpaghettiTree?
impl Tree {
	fn new() -> Tree {
		Tree {
			nodes: HashMap::new(),
			parents: HashMap::new(),
			children: HashMap::new(),
			current_id: 0,
			max_id: 0,
		}
	}

	fn add_node(&mut self, id: usize, value: usize, is_list: bool) {
		let node: Node = Node::new(value, is_list);

		self.nodes.insert(id, node);
		self.parents.insert(id, self.current_id);

		if id != self.current_id {
			self.children.entry(self.current_id).or_insert(vec![]).push(id);
		}

		self.max_id = id;
	}

	fn current_node_has_children(&self) -> bool {
		self.children.contains_key(&self.current_id)
	}

	fn goto_root(&mut self) {
		self.current_id = 0;
	}

	fn goto_parent(&mut self) {
		self.current_id = self.parents[&self.current_id];
	}

	fn goto_node(&mut self, id: usize) {
		self.current_id = id;
	}

	fn get_current_node(&self) -> &Node {
		&self.nodes[&self.current_id]
	}

	fn get_children(&self) -> Vec<usize> {
		self.children[&self.current_id].clone()
	}

	fn encapsulate_value_node(&mut self) {
		let value = self.nodes[&self.current_id].value;

		self.nodes.remove(&self.current_id);

		let node: Node = Node::new(0, true);

		self.nodes.insert(self.current_id, node);
		self.add_node(self.max_id + 1, value, false);
	}

	fn _print(&self, id: usize, level: usize, verbose: bool) {
		let mut output = "".to_string();
		
		for _ in 0 .. level * 2 {
			output.push_str("  ");
		}

		let node: &Node = &self.nodes[&id];

		if node.is_list {
			if verbose {
				output.push_str(&format!("[ (node: {}, parent: {})", id, self.parents[&id]));
			} else {
				output.push_str(&format!("["));
			}
		} else {
			if verbose {
				output.push_str(&format!("{} (node: {}, parent: {})", node.value, id, self.parents[&id]));
			} else {
				output.push_str(&format!("{}", node.value));
			}
		}

		println!("{}", output);

		if self.children.contains_key(&id) {
			for &child in &self.children[&id] {
				self._print(child, level + 1, verbose);
			}
		}

		if node.is_list {
			println!("{}", output.replace("[", "]"));
		}
	}
}

fn compare_trees(left_tree: &mut Tree, right_tree: &mut Tree) -> isize {
	println!("Comparing left tree:");
	left_tree._print(0, 0, true);
	println!("\nWith right tree:");
	right_tree._print(0, 0, true);

	left_tree.goto_root();
	right_tree.goto_root();

	// 1 -> right order, -1 -> incorrect order, 0 -> same
	fn cmp(left_tree: &mut Tree, right_tree: &mut Tree) -> isize {
		let left_node: &Node = left_tree.get_current_node();
		let right_node: &Node = right_tree.get_current_node();
		println!("CMP node {} and {}", left_tree.current_id, right_tree.current_id);

		if left_node.is_list && right_node.is_list {
			println!("LIST");

			if !left_tree.current_node_has_children() && !right_tree.current_node_has_children() {
				println!("No children in both");
				return 0;
			} else if !left_tree.current_node_has_children() {
				println!("No children in left");
				return 1;
			} else if !right_tree.current_node_has_children() {
				println!("No children in right");
				return -1;
			} else {
				let left_children = left_tree.get_children();
				let right_children = right_tree.get_children();

				for index in 0 .. left_children.len() {
					if index + 1 > right_children.len() {
						return -1;
					}

					left_tree.goto_node(left_children[index]);
					right_tree.goto_node(right_children[index]);

					let result = cmp(left_tree, right_tree);
					
					if result != 0 {
						return result;
					}
				}

				if left_children.len() < right_children.len() {
					return 1;
				}

				return 0;
			}
		} else if left_node.is_list && !right_node.is_list {
			right_tree.encapsulate_value_node();
			return cmp(left_tree, right_tree);
		} else if !left_node.is_list && right_node.is_list {
			left_tree.encapsulate_value_node();
			return cmp(left_tree, right_tree);
		} else {
			println!("Comparing values {} and {}", left_node.value, right_node.value);
			if left_node.value < right_node.value {
				return 1
			} else if left_node.value > right_node.value {
				return -1;
			} else {
				return 0;
			}
		}
	}

	return cmp(left_tree, right_tree)
}

fn parse_tree(input: String) -> Tree {
	let mut tree: Tree = Tree::new();
	let mut skip_next: bool = false;

	tree.add_node(0, 0, true);

	for (index, (c, next)) in input.chars().into_iter().tuple_windows().enumerate() {
		if skip_next {
			skip_next = false;
			continue;
		}

		let node_id = index + 1;

		if c == '[' {
			tree.add_node(node_id, 0, true);
			tree.goto_node(node_id);
		} else if c == ']' {
			tree.goto_parent();
		} else if c.is_digit(10) { // FIXME: Doesn't support more than 9!!
			let mut value: usize = c.to_digit(10).unwrap() as usize;

			if next == '0' {
				value = 10;
				skip_next = true;
			}
			
			tree.add_node(node_id, value, false);
		}
	}

	tree
}

pub fn part1(input: String) {
	let lines: Vec<String> = input
		.lines()
		.map(|line| line.parse::<String>().unwrap())
		.collect();

	let mut indices: Vec<isize> = vec![];

	for index in 0 .. lines.len() {
		if index % 3 == 1 {
			let mut left_tree: Tree = parse_tree(lines[index - 1].clone());
			let mut right_tree: Tree = parse_tree(lines[index].clone());

			indices.push(compare_trees(&mut left_tree, &mut right_tree));
		}
	}

	let mut result: usize = 0;

	for i in 0 .. indices.len() {
		if indices[i] == 1 {
			result += i + 1;
		}
	}

	println!("{}", result);
}

pub fn part2(_input: String) {
}
