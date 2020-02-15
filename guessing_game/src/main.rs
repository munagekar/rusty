extern crate rand;
use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
	let secret_number = rand::thread_rng().gen_range(0, 100);

	loop {
		println!("Enter a Guess");
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to Read Input");
		let guess: i32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};
		
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too Small"),
			Ordering::Greater => println!("Too Big"),
			Ordering::Equal => {
				println!("You Win");
				break;
			}
		}
		
	}
}
