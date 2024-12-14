use std::io::Write;
struct Robot {
	x: i128,
	y: i128,
	vx: i128,
	vy: i128,
}

fn main() {
	let width = 101;
	let height = 103;
	// let width = 11;
	// let height = 7;

	let mut grid: Vec<Vec<i32>> = Vec::new();

	for _ in 0..height {
		let mut row = Vec::new();
		for _ in 0..width {
			row.push(0);
		}
		grid.push(row);
	}

	fn reset_grid(grid: &mut Vec<Vec<i32>>) {
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				grid[y][x] = 0;
			}
		}
	}

	let input = std::fs::read_to_string("input").unwrap();
	let lines = input.lines();
	let mut matrix: Vec<Vec<u128>> = Vec::new();

	let mut robots: Vec<Robot> = Vec::new();

	for line in lines {
		let parts: Vec<String> = line
			.replace("p=", "")
			.replace("v=", "")
			.split(" ")
			.map(|s| s.to_string())
			.collect();

		let p: Vec<String> = parts
			.get(0)
			.unwrap()
			.split(",")
			.map(|s| s.to_string())
			.collect();

		let v: Vec<String> = parts
			.get(1)
			.unwrap()
			.split(",")
			.map(|s| s.to_string())
			.collect();

		let r = Robot {
			x: p.get(0).unwrap().parse().unwrap(),
			y: p.get(1).unwrap().parse().unwrap(),
			vx: v.get(0).unwrap().parse().unwrap(),
			vy: v.get(1).unwrap().parse().unwrap(),
		};

		robots.push(r);
	}

	fn print_robots(robots: &Vec<Robot>) {
		for robot in robots {
			println!("{} {} {} {}", robot.x, robot.y, robot.vx, robot.vy);
		}
	}

	fn print_grid(grid: &Vec<Vec<i32>>) {
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				print!("{}", grid[y][x]);
			}
			println!();
		}
	}

	fn update(grid: &Vec<Vec<i32>>, robot: &mut Robot) {
		if robot.x + robot.vx < grid[0].len() as i128 && robot.x + robot.vx >= 0 {
			robot.x += robot.vx;
		} else if robot.x + robot.vx >= grid[0].len() as i128 {
			robot.x = robot.x + robot.vx - grid[0].len() as i128;
		} else {
			robot.x = grid[0].len() as i128 + robot.x + robot.vx;
		}

		if robot.y + robot.vy < grid.len() as i128 && robot.y + robot.vy >= 0 {
			robot.y += robot.vy;
		} else if robot.y + robot.vy >= grid.len() as i128 {
			robot.y = robot.y + robot.vy - grid.len() as i128;
		} else {
			robot.y = grid.len() as i128 + robot.y + robot.vy;
		}
	}

	fn write_grid_to_file(grid: &Vec<Vec<i32>>, file_name: &str) {
		let mut file = std::fs::File::create(file_name).unwrap();
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				let c: char;
				if grid[y][x] > 0 {
					c = '#';
				} else {
					c = '.';
				}
				write!(file, "{}", c);
			}
			write!(file, "\n");
		}
	}

	let mut i = 0;
	loop {
		reset_grid(&mut grid);

		for robot in &mut robots {
			update(&grid, robot);
		}

		for robot in &robots {
			grid[robot.y as usize][robot.x as usize] += 1;
		}

		// check if positions are unique

		let mut unique = true;

		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				if grid[y][x] > 1 {
					unique = false;
				}
			}
		}

		if unique {
			write_grid_to_file(&grid, &format!("{}.txt", i));
			break;
		}

		println!("{}", i);

		i += 1;
	}

	fn safety_factor(grid: &Vec<Vec<i32>>) -> i32 {
		let mut sum = 0;

		let h = grid.len() / 2;
		let w = grid[0].len() / 2;

		let mut sum1 = 0;
		for y in 0..h {
			for x in 0..w {
				sum1 += grid[y][x];
			}
		}

		let mut sum2 = 0;
		for y in h + 1..grid.len() {
			for x in 0..w {
				sum2 += grid[y][x];
			}
		}

		let mut sum3 = 0;
		for y in h + 1..grid.len() {
			for x in w + 1..grid[y].len() {
				sum3 += grid[y][x];
			}
		}

		let mut sum4 = 0;
		for y in 0..h {
			for x in w + 1..grid[y].len() {
				sum4 += grid[y][x];
			}
		}

		return sum1 * sum2 * sum3 * sum4;
	}

	println!("{}", safety_factor(&grid));
}
