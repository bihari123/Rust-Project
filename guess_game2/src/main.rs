use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess_string = String::new();

        let mut guess_num: u32;

        println!("Guess a number");

        io::stdin()
            .read_line(&mut guess_string)
            .expect("Failed to read the line");

        println!("You guessed {guess_string}");

        guess_num = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("cant recognize {err}. Plz enter a number");
                continue;
            }
        };

        println!(
            "The value of guess_string is {guess_string}.\nThe value of guess_num {guess_num}"
        );

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("You won!");
                continue;
            }
        }
    }
}
