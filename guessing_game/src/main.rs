use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Guess the number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed : {guess}");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("You win"),
    }
}
