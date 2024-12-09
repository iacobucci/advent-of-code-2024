struct File {
	id: u64,
	len: u64,
	space: bool,
}

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

	let mut disk: Vec<File> = Vec::new();

	let mut space = false;
	let mut i = 0;

	for c in input.chars() {
		if space {
			let file = File {
				id: 0,
				len: char_to_int(c) as u64,
				space: space,
			};
			disk.push(file);
			i += 1;
		} else {
			let file = File {
				id: i,
				len: char_to_int(c) as u64,
				space: space,
			};
			disk.push(file);
		}

		space = !space;
	}

	fn print_disk(disk: &Vec<File>) {
		for f in disk {
			if !f.space {
				for _j in 0..f.len {
					print!("{}", f.id);
				}
			} else {
				for _j in 0..f.len {
					print!(".");
				}
			}
		}
		println!();
	}

	fn compact_spaces(disk: &mut Vec<File>) {
		let mut i = 0;
		while i < disk.len() - 1 {
			let file = disk.get(i).unwrap();
			let next_file = disk.get(i + 1).unwrap();
			if file.space && next_file.space {
				disk.get_mut(i).unwrap().len += next_file.len;
				disk.remove(i + 1);
			} else {
				i += 1;
			}
		}
	}

	fn print_spaces(disk: &Vec<File>) {
		for f in disk {
			if f.space {
				print!("{} ", f.len);
			}
		}
		println!();
	}


	fn fix_disk(disk: &mut Vec<File>, largest_id_to_try: u64) -> u64 {
		let mut found_space = false;
		let mut space_diff = 0;
		let mut file_index = 0;
		let mut space_index = 0;

		compact_spaces(disk);

		let mut smallest_id_tried = 0;

		'outer: for i in (1..disk.len()).rev() {
			let file = disk.get(i).unwrap();

			if file.space || file.id >= largest_id_to_try {
				continue;
			}

			smallest_id_tried = file.id;

			for j in 0..disk.len() {
				let space = disk.get(j).unwrap();
				if !space.space {
					continue;
				}

				if space.len >= file.len && j < i {
					found_space = true;
					space_diff = space.len - file.len;
					file_index = i;
					space_index = j;
					break 'outer;
				}
			}
		}

		if found_space {
			if space_diff > 0 {
				// swap file with space
				disk.swap(file_index, space_index);
				// at what used to be the space index, insert a new space with the remaining space
				disk.insert(
					// insert starts from 1
					space_index + 1,
					File {
						id: 0,
						len: space_diff,
						space: true,
					},
				);
				// at what used to be the file index, adjust the new space length
				if let Some(file) = disk.get_mut(file_index + 1) {
					file.len -= space_diff;
				}
			} else {
				disk.swap(file_index, space_index);
			}
		}
		return smallest_id_tried;
	}

	let mut largest_id_tried = disk.len() as u64;

	while largest_id_tried > 1 {
		largest_id_tried = fix_disk(&mut disk, largest_id_tried);
	}

	// checksum
	let mut i: u64 = 0;
	let mut sum: u64 = 0;

	for file in &disk {
		for _j in 0..file.len {
			if !file.space {
				sum += i * file.id;
			}
			i += 1;
		}
	}

	println!("{}", sum);
}
