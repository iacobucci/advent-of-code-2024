use regex::Regex;

fn main() {
	// read input file
	let input = std::fs::read_to_string("input").unwrap();

	let mut catching_do = 0;
	let mut catching_dont = 0;

	let _do = "do()";
	let _dont = "don't()";

	let mut enabled_instructions: String = "".to_string();
	let mut enabled = true;

	for c in input.chars() {

		// state automata for do() and don't()
		// this seems to go against DRY principle, but it seems to me that it would be weirder to have a function that does this, just to switch a state

		if c == _do.chars().nth(catching_do).unwrap() {
			catching_do += 1;
		} else {
			catching_do = 0;
		}

		if catching_do == _do.len()	{
			enabled = true;
			catching_do = 0;
		}


		if c == _dont.chars().nth(catching_dont).unwrap() {
			catching_dont += 1;
		} else {
			catching_dont = 0;
		}

		if catching_dont == _dont.len()	{
			enabled = false;
			catching_dont = 0;
		}


		if enabled {
			enabled_instructions.push(c);
			print!("{}", c);
		}
	}

	// same matching for enabled_instructions

	let pattern = r"mul\(\d{1,3},\d{1,3}\)";
	let re = Regex::new(pattern).unwrap();

	let matches: Vec<&str> = re
		.find_iter(&enabled_instructions)
		.map(|mat| mat.as_str())
		.collect();

	let mut sum: i32 = 0;

	for m in matches {
		let split: Vec<&str> = m.split(",").collect();
		let n1: i32 = split[0]
			.chars()
			.skip(4)
			.collect::<String>()
			.parse()
			.unwrap();
		let n2: i32 = split[1]
			.chars()
			.take(split[1].len() - 1)
			.collect::<String>()
			.parse()
			.unwrap();
		sum += n1 * n2;
	}

	println!("sum: {}", sum);
}
