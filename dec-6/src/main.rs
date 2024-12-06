enum DIR {
	UP,
	DOWN,
	LEFT,
	RIGHT,
}

fn main() {
	let mut matrix: Vec<Vec<char>>;
	let input = std::fs::read_to_string("input.test").unwrap();

	// this worked also on dec-4
	matrix = input.lines().map(|line| line.chars().collect()).collect();

	// initial position of '^'
	let mut pos = matrix
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

	// to check if adding an 'O' to a o_pos makes the guard loop, we have to check if the guard will find herself back to initial_pos and dir = UP
	fn loops(
		starting_matrix: &Vec<Vec<char>>,
		starting_pos: (i32, i32),
		starting_dir: DIR,
		o_pos: (i32, i32),
	) -> bool {
		// no recursive function, so we can shadow the loops function reference
		let mut loops = false;
		let mut pos = starting_pos.clone();
		let mut matrix = starting_matrix.clone();
		let mut dir = starting_dir;

		// put the 'O' in the matrix
		matrix[o_pos.1 as usize][o_pos.0 as usize] = 'O';

		loop {
			match dir {
				DIR::UP => {
					if ((pos.1) as i32) - 1 < 0 {
						break;
					}
					pos.1 -= 1;
				}
				DIR::DOWN => {
					if ((pos.1) as i32) + 1 >= matrix.len() as i32 {
						break;
					}
					pos.1 += 1;
				}
				DIR::LEFT => {
					if ((pos.0) as i32) - 1 < 0 {
						break;
					}
					pos.0 -= 1;
				}
				DIR::RIGHT => {
					if ((pos.0) as i32) + 1 >= matrix[0].len() as i32 {
						break;
					}
					pos.0 += 1;
				}
			}

			if (pos.0, pos.1) == starting_pos && matches!(&dir, starting_dir) {
				loops = true;

				for y in 0..matrix.len() {
					for x in 0..matrix[y].len() {
						print!("{}", matrix[y][x]);
					}
					println!();
				}
				println!();

				break;
			}

			if let Some(element) = get_matrix_element(&mut matrix, next_pos(pos, &dir)) {
				if *element == '#' || *element == 'O' {
					dir = turn_right(&dir);
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

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if !(x != initial_pos.0 as usize && y != initial_pos.1 as usize) {
				if loops(&matrix, initial_pos, DIR::UP, (x as i32, y as i32)) {
					sum += 1;
				}
			}
		}
	}

	println!("{}", sum);
}
