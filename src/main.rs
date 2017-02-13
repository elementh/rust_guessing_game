extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("Please enter your guess:");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        print!("You guessed: {}.", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            print!("Which is not a number between 1 and 100. ");
            continue
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}