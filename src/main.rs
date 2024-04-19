use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is :{secret_number}");

    println!("Enter your number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the line!");

    let guess: u32 = guess.trim().parse().expect("Please enter an integer.");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Go higher."),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Go lower."),   
    }
}
