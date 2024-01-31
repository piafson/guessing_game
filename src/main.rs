use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You made a wish: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is too small!"),
            Ordering::Greater => println!("Too big a number!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
