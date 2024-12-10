fn main() {
	let input = std::fs::read_to_string("input").unwrap();
	let lines = input.lines();
	let mut matrix: Vec<Vec<u32>> = Vec::new();

	fn char_to_int(c: char) -> u32 {
		match c {
			'0' => 0,
			'1' => 1,
			'2' => 2,
			'3' => 3,
			'4' => 4,
			'5' => 5,
			'6' => 6,
			'7' => 7,
			'8' => 8,
			'9' => 9,
			'.' => 10,
			_ => panic!("Invalid character"),
		}
	}

	for line in lines {
		matrix.push(line.chars().map(|c| char_to_int(c)).collect());
	}

	// map with key (x,y) and value (x,y)

	fn dfs(
		matrix: &Vec<Vec<u32>>,
		x: u32,
		y: u32,
		max: u32,
		total: &mut u32,
	) {
		if matrix[y as usize][x as usize] == 9 {
			*total += 1;
			return;
		}

		if x + 1 < (matrix[y as usize].len() as u32)
			&& matrix[y as usize][x as usize + 1] == max + 1
		{
			dfs(matrix, x + 1, y, max + 1, total);
		}

		if x as i32 - 1 >= 0 && matrix[y as usize][x as usize - 1] == max + 1 {
			dfs(matrix, x - 1, y, max + 1, total);
		}

		if y + 1 < (matrix.len() as u32) && matrix[y as usize + 1][x as usize] == max + 1 {
			dfs(matrix, x, y + 1, max + 1, total);
		}

		if y as i32 - 1 >= 0 && matrix[y as usize - 1][x as usize] == max + 1 {
			dfs(matrix, x, y - 1, max + 1, total);
		}
	}

	let mut total = 0;

	for y in 0..matrix.len() {
		for x in 0..matrix[y].len() {
			if matrix[y][x] == 0 {
				dfs(&matrix, x as u32, y as u32, 0, &mut total);
			}
		}
	}

	println!("{}", total);
}
