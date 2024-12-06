enum DIR {
	UP,
	DOWN,
	LEFT,
	RIGHT,
}

fn main() {
	let matrix: Vec<Vec<char>>;
	let input = std::fs::read_to_string("input").unwrap();

	// this worked also on dec-4
	matrix = input.lines().map(|line| line.chars().collect()).collect();

	// initial position of '^'
	let pos = matrix
		.iter()
		// with enumerate we make the iteration over the rows keeping an index
		.enumerate()
		// we bring y and row in a closure
		.find_map(|(y, row)| {
			// there we iterate over the columns of each row, returning the result of the first match
			row.iter()
				// checking if the charater is '^', position returns an Option<usize>
				.position(|&c| c == '^')
				// we map the Option<usize> to an Option<(usize, usize)> with the y value that we have still in scope
				.map(|x| (x as i32, y as i32))
		})
		// we unwrap the Option<(usize, usize)> to (usize, usize). this is a possible failure point
		.unwrap();

	let initial_pos = pos;

	fn get_matrix_element(matrix: &mut Vec<Vec<char>>, (x, y): (i32, i32)) -> Option<&mut char> {
		if x < 0 || y < 0 {
			return None;
		}

		let y = y as usize;
		let x = x as usize;

		if y >= matrix.len() || x >= matrix[y].len() {
			return None;
		}

		Some(&mut matrix[y][x])
	}

	fn turn_right(dir: &DIR) -> DIR {
		match dir {
			DIR::UP => DIR::RIGHT,
			DIR::DOWN => DIR::LEFT,
			DIR::LEFT => DIR::UP,
			DIR::RIGHT => DIR::DOWN,
		}
	}

	fn next_pos(pos: (i32, i32), dir: &DIR) -> (i32, i32) {
		match dir {
			DIR::UP => (pos.0, pos.1 - 1),
			DIR::DOWN => (pos.0, pos.1 + 1),
			DIR::LEFT => (pos.0 - 1, pos.1),
			DIR::RIGHT => (pos.0 + 1, pos.1),
		}
	}

	fn loops(
		starting_matrix: &Vec<Vec<char>>,
		starting_pos: (i32, i32),
		starting_dir: DIR,
		o_pos: (i32, i32),
	) -> bool {
		// no recursive function, so we can shadow the loops function reference
		let loops = false;
		let mut pos = starting_pos.clone();
		let mut matrix = starting_matrix.clone();
		let mut dir = starting_dir;

		// put the 'O' in the matrix
		matrix[o_pos.1 as usize][o_pos.0 as usize] = 'O';
		let mut right_turns_at_an_o = 0;

		loop {
			match dir {
				DIR::UP => {
					if ((pos.1) as i32) - 1 < 0 {
						break;
					}
					pos = next_pos(pos, &dir);
				}
				DIR::DOWN => {
					if ((pos.1) as i32) + 1 >= matrix.len() as i32 {
						break;
					}
					pos = next_pos(pos, &dir);
				}
				DIR::LEFT => {
					if ((pos.0) as i32) - 1 < 0 {
						break;
					}
					pos = next_pos(pos, &dir);
				}
				DIR::RIGHT => {
					if ((pos.0) as i32) + 1 >= matrix[0].len() as i32 {
						break;
					}
					pos = next_pos(pos, &dir);
				}
			}

			if let Some(element) = get_matrix_element(&mut matrix, next_pos(pos, &dir)) {
				if *element == '#' {
					dir = turn_right(&dir);
				}
				if *element == 'O' {
					dir = turn_right(&dir);
					right_turns_at_an_o += 1;

					// given that you can only bump in the O in 4 different directions, each of those direction might be a faux positive, and lead to a non looping path. if we check that the guard bumps into the O at least 5 times, we are sure that she will loop
					if right_turns_at_an_o == 5 {
						return true;
					}
				}
			}
		}

		return loops;
	}

	// part 1
	// let mut sum = 0;

	// for y in 0..matrix.len() {
	// 	for x in 0..matrix[y].len() {
	// 		if matrix[y][x] == 'X' {
	// 			sum += 1;
	// 		}
	// 	}
	// }

	// println!("{}", sum);

	// part 2

	let mut sum = 0;

	let total_to_check = matrix.len() * matrix[0].len() - 1;

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if !(x == initial_pos.0 as usize && y == initial_pos.1 as usize) {
				print!(
					"checking {} of {} ({}%)",
					x + y * matrix[0].len(),
					total_to_check,
					(x + y * matrix[0].len()) as f32 / total_to_check as f32 * 100.0
				);
				if loops(&matrix, initial_pos, DIR::UP, (x as i32, y as i32)) {
					sum += 1;
					print!(": true\n");
				} else {
					print!(": false\n");
				}
			}
		}
	}

	println!("{}", sum);
}
