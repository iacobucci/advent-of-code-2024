use itertools::Itertools;

fn main() {
	let input = std::fs::read_to_string("input").unwrap();

	fn char_to_int(c: char) -> i32 {
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
			_ => panic!("Invalid character"),
		}
	}

	let mut disk: Vec<i64> = Vec::new();

	let mut file = true;
	let mut i = 0;

	for c in input.chars() {
		if file {
			for _j in 0..char_to_int(c) {
				disk.push(i);
			}
			i += 1;
		} else {
			for _j in 0..char_to_int(c) {
				disk.push(-1);
			}
		}

		file = !file;
	}

	println!(
		"{}",
		Itertools::join(&mut disk.iter(), "").replace("-1", ".")
	);

	// swap every last positive number to the first negative number in the list, until all the negative numbers are at the end of the list

	let mut first_neg = 0;

	for i in (0..disk.len()).rev() {
		for j in 0..disk.len() {
			if disk.get(j).unwrap() < &0 {
				first_neg = j;
				break;
			}
		}
		if disk.get(i).unwrap() > &0 {
			if i <= first_neg {
				break;
			}
			disk.swap(i, first_neg);
		}
	}

	println!(
		"{}",
		Itertools::join(&mut disk.iter(), "").replace("-1", ".")
	);

	// checksum
	let mut i = 0;
	let mut sum: u64 = 0;
	loop {
		let id = disk.get(i).unwrap();
		if id < &0 {
			break;
		}

		sum += *id as u64 * i as u64;

		i += 1;
	}

	println!("{}", sum);
}
