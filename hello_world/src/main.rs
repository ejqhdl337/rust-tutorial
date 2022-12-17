extern crate rand;

use rand::Rng;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();

    loop {
        print!("Guess the number: ");
        io::stdout().flush().expect("fail to flush");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error with reading line");
        guess.pop();

        let quess_number: i32 = guess.parse().unwrap();

        if (quess_number == secret_number) {
            println!("srcret number was {}! you win", secret_number);
            break;
        } else if (quess_number > secret_number) {
            println!("Down!");
        } else if (quess_number < secret_number) {
            println!("Up!")
        }
        guess.clear();
    }
}
