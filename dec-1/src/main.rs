fn main() {
	// read input file
	let input = std::fs::read_to_string("input").unwrap();

	let mut list1: Vec<i32> = Vec::new();
	let mut list2: Vec<i32> = Vec::new();

	// cycle through each line
	for line in input.lines() {
		// split the line into two parts
		let parts: Vec<&str> = line.split("   ").collect();

		// parse the two parts into integers
		let num1: i32 = parts[0].parse().unwrap();
		let num2: i32 = parts[1].parse().unwrap();

		// add the two numbers to the lists
		list1.push(num1);
		list2.push(num2);
	}

	list1.sort();
	list2.sort();

	let mut sum = 0;

	for i in 0..list1.len() {
		sum += (list1[i] - list2[i]).abs();
	}

	println!("{}", sum);

	// part 2

	let mut sum2 = 0;

	for i1 in 0..list1.len() {
		for i2 in 0..list2.len() {
			if list1[i1] == list2[i2] {
				sum2 += list1[i1];
			}
		}
	}

	println!("{}", sum2);
}
