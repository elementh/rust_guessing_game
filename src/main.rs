extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = Guess::new(rand::thread_rng().gen_range(1, 101));

    struct Guess {
        value: u32,
    }
    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess {
                value: value,
            }
        }
        pub fn value(&self) -> u32 {
            self.value
        }
    }
    //    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("Please enter your guess:");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        print!("You guessed: {}, ", guess.trim());

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number.value()) {
            Ordering::Less => println!("which is too small!"),
            Ordering::Greater => println!("which is too big!"),
            Ordering::Equal => {
                println!("which is right!");
                println!("You win!");
                break;
            },
        }
    }
}