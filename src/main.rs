use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100); // Generates random number 1-100
    println!("Secret number is: {}", secret_number);

    

    loop { //
        println!("Input your guess!");
        let mut guess = String::new();

    io::stdin()
        .read_line( &mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}","Too Small!".red()),
        Ordering::Greater => println!("{}","Too big!".red()),
        Ordering::Equal => {
            println!("{}","You won!".green());
            break;}
    }
    }
}
