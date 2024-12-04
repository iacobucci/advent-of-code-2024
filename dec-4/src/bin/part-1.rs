const XMAS: &str = "XMAS";

enum DIR {
	R,
	L,
	U,
	D,
	UR,
	UL,
	DR,
	DL,
}

fn main() {
	let matrix: Vec<Vec<char>>;

	let input = std::fs::read_to_string("input").unwrap();

	matrix = input.lines().map(|line| line.chars().collect()).collect();

	let mut counts: [i32; 8] = [0; 8];

	let mut sum = 0;

	for y in 0..matrix.len() {
		'inner: for x in 0..matrix[y].len() {
			for count in &mut counts {
				*count = 0;
			}

			// Initial
			if matrix.get(y).and_then(|row| row.get(x)) == XMAS.chars().nth(0).as_ref() {
				for count in &mut counts {
					*count += 1;
				}
			} else {
				continue 'inner;
			}

			// Directions

			for i in 1..XMAS.len() {
				let c = XMAS.chars().nth(i).unwrap();

				// r
				if matrix.get(y).and_then(|row| row.get(x + i)) == Some(&c) {
					counts[DIR::R as usize] += 1;
				}

				// l
				if x >= i && matrix.get(y).and_then(|row| row.get(x - i)) == Some(&c) {
					counts[DIR::L as usize] += 1;
				}

				// u
				if y >= i && matrix.get(y - i).and_then(|row| row.get(x)) == Some(&c) {
					counts[DIR::U as usize] += 1;
				}

				// d
				if matrix.get(y + i).and_then(|row| row.get(x)) == Some(&c) {
					counts[DIR::D as usize] += 1;
				}

				// ur
				if y >= i && matrix.get(y - i).and_then(|row| row.get(x + i)) == Some(&c) {
					counts[DIR::UR as usize] += 1;
				}

				// ul
				if y >= i && x >= i && matrix.get(y - i).and_then(|row| row.get(x - i)) == Some(&c)
				{
					counts[DIR::UL as usize] += 1;
				}

				// dr
				if matrix.get(y + i).and_then(|row| row.get(x + i)) == Some(&c) {
					counts[DIR::DR as usize] += 1;
				}

				// dl
				if x >= i && matrix.get(y + i).and_then(|row| row.get(x - i)) == Some(&c) {
					counts[DIR::DL as usize] += 1;
				}
			}

			for count in &counts {
				if *count == XMAS.len() as i32 {
					sum += 1;
				}
			}
		}
	}

	println!("{}", sum);
}
