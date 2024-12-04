// this time we enumerate all the possible matches, for the fact that theyre in a small number

#[rustfmt::skip]
const MAS: [[[char; 3]; 3]; 4] = [
	[
	['M', ' ', 'M'],
	[' ', 'A', ' '],
	['S', ' ', 'S'], ],
	[
	['S', ' ', 'M'],
	[' ', 'A', ' '],
	['S', ' ', 'M'], ],
	[
	['S', ' ', 'S'],
	[' ', 'A', ' '],
	['M', ' ', 'M'], ],
	[
	['M', ' ', 'S'],
	[' ', 'A', ' '],
	['M', ' ', 'S'], ], ];

fn main() {
	let matrix: Vec<Vec<char>>;
	let input = std::fs::read_to_string("input").unwrap();

	matrix = input.lines().map(|line| line.chars().collect()).collect();

	let mut sum = 0;

	// given that we have to match for MAS, with A being in the center, we can remove the borders of the matrix and be sure that we will not go out of bounds

	for y in 1..matrix.len() - 1 {
		'inner: for x in 1..matrix[y].len() - 1 {
			// all the 4 variations of X-MAS
			for i in 0..4 {
				let mut matched = true;

				for j in 0..3 {
					for k in 0..3 {
						// with some De Morgan magic, if every neighbor of the element matrix[y][x] is not the same as the corresponding one in at lest one of the variations of the X-MASes, provided that the X-MAS[i][j][k] is not a space, then we can be sure that the matrix[y][x] is not the center of the X-MAS

						if matrix.get(y + j - 1).and_then(|row| row.get(x + k - 1))
							!= Some(&MAS[i][j][k])
							&& MAS[i][j][k] != ' '
                        {
                            matched = false;
                            break;
                        }
                    }
                    if !matched {
                        break;
                    }
                }

                if matched {
                    sum += 1;
					continue 'inner;
                }

			}
		}
	}

	println!("{}", sum);
}
