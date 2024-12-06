struct Rule {
	before: i32,
	after: i32,
}

fn main() {
	let mut rules: Vec<Rule> = Vec::new();
	let mut updates: Vec<Vec<i32>> = Vec::new();

	let input = std::fs::read_to_string("input").unwrap();

	let mut reading_updates = false;

	for line in input.lines() {
		if line.is_empty() {
			reading_updates = true;
			continue;
		}

		if !reading_updates {
			let split = line.split("|").collect::<Vec<&str>>();
			let rule = Rule {
				before: split[0].parse().unwrap(),
				after: split[1].parse().unwrap(),
			};

			rules.push(rule);
		} else {
			let split = line.split(",").collect::<Vec<&str>>();
			updates.push(Vec::new());
			for s in split {
				updates.last_mut().unwrap().push(s.parse().unwrap());
			}
		}
	}

	let check_if_follows_rules = |n1: i32, n2: i32| -> bool {
		let mut follows = true;
		for rule in &rules {
			if n1 == rule.after && n2 == rule.before {
				follows = false;
				break;
			}
		}
		follows
	};

	let mut followingupdates: Vec<Vec<i32>> = Vec::new();
	let mut notfollowingupdates: Vec<Vec<i32>> = Vec::new();

	for update in &updates {
		let mut follows = true;
		let len = update.len();
		followingupdates.push(update.clone());
		for i in 0..len - 1 {
			if !check_if_follows_rules(update[i], update[i + 1]) {
				follows = false;
				followingupdates.pop();
				break;
			}
		}

		if !follows {
			notfollowingupdates.push(update.clone());
		}
	}

	let mut sum = 0;

	// part 1
	for update in followingupdates {
		for i in 0..update.len() {
			if i == (update.len() / 2) {
				sum += update[i];
			}
		}
	}
	println!("{}", sum);

	sum = 0;

	// part 2
	for mut update in notfollowingupdates {
		update.sort_by(|a, b| {
			if check_if_follows_rules(*a, *b) {
				std::cmp::Ordering::Less 
			} else if check_if_follows_rules(*b, *a) {
				std::cmp::Ordering::Greater 
			} else {
				std::cmp::Ordering::Equal 
			}
		});

		let len = update.len();
		for i in 0..len {
			if i == (len / 2) {
				sum += update[i];
			}
		}
	}

	println!("{}", sum);
}
