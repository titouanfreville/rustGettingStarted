extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 301);
    let mut guess = String::new();
    let mut guess_nb: u32;

    println!("Guess the number!");
    
    loop {
        
        println!("Please type your guess: ");

        io::stdin().read_line(&mut guess).expect("Failed to read your guess :'(");

        guess_nb = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess = String::new();

        match guess_nb.cmp(&secret_number) {
            Ordering::Less => println!("Your guess: {} is to small. -", guess),
            Ordering::Greater => println!("Your guess: {} is to big. +", guess),
            Ordering::Equal => {println!("Your guess: {} is correct. =.\nYou won!", guess);break;}
        }

    }
}
