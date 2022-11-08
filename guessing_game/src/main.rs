use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess!");
        println!("Ths secret number is {}", secret_number);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Less => println!("{}", "Too small!".red()),
        }
    }
}
