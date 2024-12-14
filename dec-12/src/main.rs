use std::collections::HashSet;

fn main() {
	let input = std::fs::read_to_string("input.test.4").unwrap();
	let lines = input.lines();
	let mut matrix: Vec<Vec<char>> = Vec::new();

	for line in lines {
		matrix.push(line.chars().collect());
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[y].len() {
			print!("{}", matrix[y][x]);
		}
		println!();
	}

	fn aligned(p1: (usize, usize), tup: ((usize, usize), (usize, usize))) -> bool {
		let mut aligned = true;

		let p2 = tup.0;
		let p3 = tup.1;

		// horizontal
		if p3.0 == p2.0 {
			aligned = p1.0 == p2.0;
		}
		// vertical
		else if p3.1 == p2.1 {
			aligned = p1.1 == p2.1;
		} else {
			aligned = false;
		}

		return aligned;
	}

	fn flood_fill(
		matrix: &Vec<Vec<char>>,
		visited: &mut HashSet<(usize, usize)>,
		x: usize,
		y: usize,
	) -> (usize, usize, usize) {
		let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
		let mut stack = vec![(x, y)];
		let mut area = 0;
		let mut edges: Vec<((usize, usize), (usize, usize))> = vec![];
		let mut perimeter = 0;
		let mut num_sides = 0;
		let char_region = matrix[x][y];
		let rows = matrix.len();
		let cols = matrix[0].len();

		while let Some((cx, cy)) = stack.pop() {
			if visited.contains(&(cx, cy)) {
				continue;
			}
			visited.insert((cx, cy));
			area += 1;

			for i in 0..directions.len() {
				let (dx, dy) = directions[i];
				let nx = cx as isize + dx;
				let ny = cy as isize + dy;

				if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
					perimeter += 1;
					continue;
				}

				let nx = nx as usize;
				let ny = ny as usize;

				if matrix[nx][ny] != char_region {
					perimeter += 1;
				} else if !visited.contains(&(nx, ny)) {
					stack.push((nx, ny));
				}
			}
		}

		(area, perimeter, num_sides)
	}

	fn find_regions(matrix: Vec<Vec<char>>) -> Vec<(char, usize, usize, usize)> {
		let mut visited = HashSet::new();
		let mut regions = vec![];

		for i in 0..matrix.len() {
			for j in 0..matrix[0].len() {
				if !visited.contains(&(i, j)) {
					let (area, perimeter, num_sides) = flood_fill(&matrix, &mut visited, i, j);
					regions.push((matrix[i][j], area, perimeter, num_sides));
				}
			}
		}

		regions
	}

	let mut sum = 0;

	let regions = find_regions(matrix);
	for (char_region, area, perimeter, sides) in regions {
		sum += area * perimeter;
		println!(
			"Regione '{}': Area = {}, Perimetro = {}, Lati = {}",
			char_region, area, perimeter, sides
		);
	}

	println!("{}", sum);
}
