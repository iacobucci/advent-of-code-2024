fn main() {
	let input = std::fs::read_to_string("input").unwrap();
	let lines = input.lines();

	// iterate every 4 lines

	let mut sum = 0;

	let mut i: i128 = 0;
	let mut ax: i128 = 0;
	let mut ay: i128 = 0;
	let mut bx: i128 = 0;
	let mut by: i128 = 0;
	let mut px: i128 = 0;
	let mut py: i128 = 0;

	for line in lines {
		i += 1;

		if i == 1 {
			let parts: Vec<String> = line
				.replace("Button A: ", "")
				.split(", ")
				.map(|s| s.replace("+", "").to_string())
				.collect();

			ax = parts[0][1..].parse::<i128>().unwrap();
			ay = parts[1][1..].parse::<i128>().unwrap();
		}

		if i == 2 {
			let parts: Vec<String> = line
				.replace("Button B: ", "")
				.split(", ")
				.map(|s| s.replace("+", "").to_string())
				.collect();

			bx = parts[0][1..].parse::<i128>().unwrap();
			by = parts[1][1..].parse::<i128>().unwrap();
		}

		if i == 3 {
			let parts: Vec<String> = line
				.replace("Prize: ", "")
				.split(", ")
				.map(|s| s.replace("=", "").to_string())
				.collect();

			px = parts[0][1..].parse::<i128>().unwrap() + 10000000000000;
			py = parts[1][1..].parse::<i128>().unwrap() + 10000000000000;
		}

		if i == 4 {
			// solve linear equation:
			// px = (ax * a + bx * b)
			// py = (ay * a + by * b)
			// find a and b if there is a solution

			let det = ax * by - ay * bx;

			if det == 0 {
			} else {
				let is_solution_integer =
					(px * by - py * bx) % det == 0 && (ax * py - ay * px) % det == 0;

				if is_solution_integer {
					let a = (px * by - py * bx) / det;
					let b = (ax * py - ay * px) / det;

					sum += 3 * a + b;
				}
			}

			i = 0;
		}
	}

	println!("{}", sum);
}
