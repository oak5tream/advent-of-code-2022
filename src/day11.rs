struct Monkey {
	items: Vec<usize>,
	op_add: usize,
	op_mul: usize,
	op_sqrt: usize,
	test: usize,
	throw_true: usize,
	throw_false: usize,
	inspected: usize,
}

impl Monkey {
	fn new(items: Vec<usize>, op_add: usize, op_mul: usize, op_sqrt: usize, test: usize, throw_true: usize, throw_false: usize) -> Monkey {
		Monkey {
			items,
			op_add,
			op_mul,
			op_sqrt,
			test,
			throw_true,
			throw_false,
			inspected: 0,
		}
	}

	fn has_items(&self) -> bool {
		self.items.len() > 0
	}

	fn inspect(&mut self, reduced_worry_levels: bool) -> (usize, usize) {
		self.inspected += 1;
		let mut item = self.items.pop().unwrap();

		if self.op_add > 0 {
			item += self.op_add;
		} else if self.op_mul > 0 {
			item *= self.op_mul;
		} else if self.op_sqrt > 0 {
			item *= item;
		}

		if reduced_worry_levels {
			item /= 3;
		}

		let throw_to = if item % self.test == 0 { self.throw_true } else { self.throw_false };

		(item, throw_to)
	}
}

fn _create_test_data() -> Vec<Monkey> {
	let mut monkeys: Vec<Monkey> = vec![];

	monkeys.push(Monkey::new(vec![98, 79], 0, 19, 0, 23, 2, 3));
	monkeys.push(Monkey::new(vec![74, 75, 65, 54], 6, 0, 0, 19, 2, 0));
	monkeys.push(Monkey::new(vec![97, 60, 79], 0, 0, 2, 13, 1, 3));
	monkeys.push(Monkey::new(vec![74], 3, 0, 0, 17, 0, 1));

	monkeys
}

fn create_data() -> Vec<Monkey> {
	let mut monkeys: Vec<Monkey> = vec![];

	monkeys.push(Monkey::new(vec![52, 89, 98], 0, 2, 0, 5, 6, 1));
	monkeys.push(Monkey::new(vec![78, 57, 92, 80, 95, 57], 0, 13, 0, 2, 2, 6));
	monkeys.push(Monkey::new(vec![83, 92, 51, 75, 97, 74, 82], 5, 0, 0, 19, 7, 5));
	monkeys.push(Monkey::new(vec![76, 68, 51, 88, 97], 6, 0, 0, 7, 0, 4));
	monkeys.push(Monkey::new(vec![63], 1, 0, 0, 17, 0, 1));
	monkeys.push(Monkey::new(vec![63, 51, 91, 94], 4, 0, 0, 13, 4, 3));
	monkeys.push(Monkey::new(vec![83, 98, 68, 74, 71, 94, 54, 61], 2, 0, 0, 3, 2, 7));
	monkeys.push(Monkey::new(vec![56, 90], 0, 0, 1, 11, 3, 5));

	monkeys
}

fn _print_monkeys(monkeys: Vec<Monkey>) {
	for i in 0 .. monkeys.len() {
		let mut items = monkeys[i].items.clone();
		items.reverse();
		println!("Monkey {}: {:?}", i, items);
	}
}

fn solve(reduced_worry_levels: bool, rounds: usize) -> usize {
	let mut monkeys: Vec<Monkey> = create_data();
	let mut modulo: usize = 0;

	for i in 0 .. monkeys.len() {
		modulo = if modulo == 0 { monkeys[i].test } else { modulo * monkeys[i].test };
	}

	for _ in 0 .. rounds {
		for i in 0 .. monkeys.len() {

			while monkeys[i].has_items() {
				let (item, throw_to) = monkeys[i].inspect(reduced_worry_levels);
				monkeys[throw_to].items.insert(0, item % modulo);
				
			}
		}
	}

	let mut inspections: Vec<usize> = vec![];
	for i in 0 .. monkeys.len() {
		inspections.push(monkeys[i].inspected);
	}
	inspections.sort();
	inspections.reverse();

	inspections[0] * inspections[1]
}

pub fn part1(_ignored: String) {
	println!("{:?}", solve(true, 20));
}

pub fn part2(_ignored: String) {
	println!("{:?}", solve(false, 10000));
}
