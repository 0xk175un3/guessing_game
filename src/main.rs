extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("From 1 to 100");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Error while try to read strign");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Nope, it's more than this"),
            Ordering::Greater => println!("Nope, less"),
            Ordering::Equal   => {
                println!("Nice one!");
                break;
            }
        }
    }
}
