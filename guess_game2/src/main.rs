use std::io;

fn main() {
    println!("Welcome to the Guess Game!");
    loop {
        println!("\nPlease Enter the Input.");

        let mut guess_string = String::new();
        io::stdin()
            .read_line(&mut guess_string)
            .expect("Failed to read the Line");

        let guess: u32 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nYou have entered {guess_string}. Plz Enter a number");
                continue;
            }
        };

        println!("\nYou have entered {guess}")
    }
}
