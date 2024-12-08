fn main() {
	let input = std::fs::read_to_string("input").unwrap();
	let lines = input.lines();
	let mut matrix: Vec<Vec<char>> = Vec::new();
	let mut antinodes: Vec<Vec<char>> = Vec::new();

	for line in lines {
		matrix.push(line.chars().collect());
		antinodes.push(vec!['.'; line.len()]);
	}

	let rows = matrix.len();
	let cols = matrix[0].len();

	for y1 in 0..rows {
		for x1 in 0..cols {
			if matrix[y1][x1] == '.' {
				continue;
			}
			for y2 in 0..rows {
				for x2 in 0..cols {
					if matrix[y1][x1] == matrix[y2][x2] && (x1 != x2 || y1 != y2) {
						let x_diff = x1 as isize - x2 as isize;
						let y_diff = y1 as isize - y2 as isize;

						let mut antinode_x = x1 as isize;
						let mut antinode_y = y1 as isize;

						loop {
							// Aggiorna la matrice degli antinodi
							antinodes[antinode_y as usize][antinode_x as usize] = 'X';

							antinode_x += x_diff;
							antinode_y += y_diff;

							// Interrompi il ciclo se siamo fuori dai limiti
							if antinode_x < 0
								|| antinode_y < 0 || antinode_x >= cols as isize
								|| antinode_y >= rows as isize
							{
								break;
							}
						}
					}
				}
			}
		}
	}

	let mut sum = 0;
	for y in 0..antinodes.len() {
		for x in 0..antinodes[y].len() {
			if antinodes[y][x] == 'X' {
				sum += 1;
			}
			print!("{}", antinodes[y][x]);
		}

		print!("\t");

		for x in 0..matrix[y].len() {
			print!("{}", matrix[y][x]);
		}
		println!();
	}
	println!("{}", sum);
}
