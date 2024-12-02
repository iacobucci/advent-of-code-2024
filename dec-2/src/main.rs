fn main() {
	// read input file
	let input = std::fs::read_to_string("input").unwrap();

	let mut safe: i32 = 0;
	let mut safeline: bool = false;
	let mut i: i32;

	let mut num: i32;
	let mut oldnum: i32;
	let mut direction: bool = true;

	let min_diff: i32 = 1;
	let max_diff: i32 = 3;

	for line in input.lines() {
		let levels: Vec<&str> = line.split(" ").collect();

		// create a copy of parts for every part of the line
		// this solution is kinda bruteforce-y, but it takes into account many different situations in one go

		'outer: for j in 0..levels.len() {
			let mut inner_levels = levels.clone();

			// remove the j-th part from inner_parts
			inner_levels.remove(j);

			oldnum = 0;
			i = 0;
			'inner: for part in inner_levels {
				{
					num = part.parse().unwrap();

					if i == 0 {
						safeline = true;
					}

					if i == 1 {
						if !((num - oldnum).abs() >= min_diff && (num - oldnum).abs() <= max_diff) {
							safeline = false;
						}
						direction = num > oldnum;
					}

					if i > 1 {
						if !((num > oldnum) == direction
							&& (num - oldnum).abs() >= min_diff
							&& (num - oldnum).abs() <= max_diff)
						{
							safeline = false;
						}
						direction = num > oldnum;
					}

					i += 1;
					oldnum = num;

					if !safeline {
						break 'inner;
					}
				}
			}

			if safeline {
				safe += 1;
				// count a line as safe only once
				break 'outer;
			}
		}
	}

	println!("{}", safe);
}
