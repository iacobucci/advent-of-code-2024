fn main() {
	#[derive(Clone)]
	struct Stone {
		digit: u128,
	}

	let input = std::fs::read_to_string("input").unwrap();

	let mut stones: Vec<Stone> = Vec::new();

	input.split(" ").for_each(|s| {
		stones.push(Stone {
			digit: s.parse::<u128>().unwrap(),
		});
	});

	fn rule1(stone: &Stone) -> Option<Vec<Stone>> {
		if stone.digit == 0 {
			return Some(vec![Stone { digit: 1 }]);
		} else {
			return None;
		}
	}

	fn rule2(stone: &Stone) -> Option<Vec<Stone>> {
		let decimal_representation = stone.digit.to_string();
		if decimal_representation.len() % 2 == 0 {
			let parts = decimal_representation.split_at(decimal_representation.len() / 2);

			let first_half = parts.0.parse::<u128>().unwrap();
			let second_half = parts.1.parse::<u128>().unwrap();

			return Some(vec![
				Stone { digit: first_half },
				Stone { digit: second_half },
			]);
		} else {
			return None;
		}
	}

	fn rule3(stone: &Stone) -> Option<Vec<Stone>> {
		return Some(vec![Stone {
			digit: stone.digit * 2024,
		}]);
	}

	fn print_stones(stones: &Vec<Stone>) {
		for stone in stones {
			print!("{} ", stone.digit);
		}
		println!("")
	}

	print_stones(&stones);

	for _i in 0..75 {
		let mut new_stones: Vec<Stone> = Vec::new();

		for stone in stones.clone() {
			if let Some(new) = rule1(&stone) {
				// print!("rule1: ");
				// print_stones(&new);
				new_stones.extend(new);
			} else if let Some(new) = rule2(&stone) {
				// print!("rule1: ");
				// print_stones(&new);
				new_stones.extend(new);
			} else if let Some(new) = rule3(&stone) {
				// print!("rule3: ");
				// print_stones(&new);
				new_stones.extend(new);
			}
		}

		stones = new_stones;
		// print_stones(&stones);
	}

	println!("{}", stones.len());
}
