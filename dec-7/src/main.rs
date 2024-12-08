use indexmap::map::IndexMap as Map;

fn main() {
	let mut lines: Map<u64, Vec<u64>> = Map::new();

	let input = std::fs::read_to_string("input").unwrap();

	for line in input.lines() {
		let split = line.split(":").collect::<Vec<&str>>();

		let key = split[0].parse::<u64>().unwrap();

		let values = split[1]
			.split_whitespace()
			.map(|v| v.parse::<u64>().expect("Failed to parse value"))
			.collect::<Vec<u64>>();

		lines.insert(key, values);
	}

	fn sum_and_mul_and_concat(key: u64, result: u64, values: &Vec<u64>) -> bool {
		if result == key && values.is_empty() {
			return true;
		}
		if !values.is_empty() {
			if sum_and_mul_and_concat(key, result + values[0], &values[1..].to_vec()) {
				return true;
			}

			if sum_and_mul_and_concat(key, result * values[0], &values[1..].to_vec()) {
				return true;
			}

			if sum_and_mul_and_concat(
				key,
				(result.to_string() + values[0].to_string().as_str())
					.parse::<u64>()
					.unwrap(),
				&values[1..].to_vec(),
			) {
				return true;
			}
		}

		return false;
	}

	let mut sum = 0;

	for (key, values) in lines {
		// assume we have more than one value
		if sum_and_mul_and_concat(key, values[0], &values[1..].to_vec()) {
			println!("{}", key);
			sum += key;
		}
	}
}
