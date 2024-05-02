use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..=100);
    loop {
        println!("Please input your number: ");

        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Falied to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {guess}");
        match guess.cmp(& secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The answer is {guess}!");
                break;
            }
        }
    }

}
