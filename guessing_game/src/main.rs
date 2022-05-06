extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn check_guess(expected: u32) -> bool {
    let mut guess = String::new();

    println!("Please type your guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your guess :'(");

    let guess_nb: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    guess = String::new();

    match guess_nb.cmp(&expected) {
        Ordering::Less => println!("Your guess: {} is to small. -", guess),
        Ordering::Greater => println!("Your guess: {} is to big. +", guess),
        Ordering::Equal => {
            println!("Your guess: {} is correct.\nYou won!", guess);
            return true;
        }
    }

    false
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 301);
    println!("Guess the number!");

    loop {
        if check_guess(secret_number) {
            break;
        }
    }
}
