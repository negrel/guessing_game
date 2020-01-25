use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
use std::mem::drop;
use std::process::exit;

fn main() {
	// Use to read user input;
	let mut reader = String::new();
	// Game variable
	let mut guess: u32;			// User input (is guess for mystery number).
	let mut attempt: u32 = 0; 		// Attempt counter.
	let max_attempt: u32; 	// Max attempt.

	// Banner prompt
	println!("### Guess the number ! ###");
	
	// --------------------------------------------------------------------- //
	// Range of mystery number [min, max[.
	// --------------------------------------------------------------------- //
	println!("Enter the mystery number range.");

	// Min of mystery number
	println!("Min : ");
	io::stdin().read_line(&mut reader)
		.expect("Failed to read line.");
	let min = parse_to_u32(&mut reader);

	// Max of mystery number
	println!("Max : ");
	io::stdin().read_line(&mut reader)
		.expect("Failed to read line.");
	let max = parse_to_u32(&mut reader);

	
	// Mystery number.
	let res = get_random(min, max);
	
	// Clear memory of range variables.
	drop(min);
	drop(max);
	// --------------------------------------------------------------------- //

	// Number of attempts.
	println!("How many attempt do you want ?");
	io::stdin().read_line(&mut reader)
		.expect("Failed to read line.");
	// Parse to interger.
	max_attempt = parse_to_u32(&mut reader);
	
	while attempt < max_attempt {

		// Prompt user to enter a number
		println!("Please input your guess.");

		// Read the user input
		io::stdin().read_line(&mut reader)
			.expect("Failed to read line.");
		guess = parse_to_u32(&mut reader);
		
		// Check the user input	
		match guess.cmp(&res) {
			// The user input is smaller than the mystery number
			Ordering::Less => println!("Too small!"),
			// The user input is greater than the mystery number
			Ordering::Greater => println!("Too big!"),
			// The user input is equal to the mystery number
			Ordering::Equal => {
				println!("You win!");
				exit(0);
				break;
			}
		}
		// Increment attempt counter.
		attempt += 1;
	}

	println!("You loose...");
	exit(0);
}

// Return a random number from the given range
fn get_random(min: u32, max: u32) -> u32 {
	return thread_rng().gen_range(min, max);
}

// Parse the given string to u32
fn parse_to_u32(str: &mut String) -> u32 {
	// Parse the string to u32
	let parsed = match str.trim().parse::<u32>() {
		Ok(num) => num,
		Err(e) => panic!(e)
	};

	// Clean the reader
	*str = String::new();

	return parsed;
}
