// std: Standard Library
// io: (input/output)

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("GUESSING GAME");

    // gen_range(start..end): (1..101) equivalent to (1..=100)

    let secret_number = rand::thread_rng().gen_range(1..101); // 1 to 100

    println!("Secret Number: {}", secret_number);

    loop {
        println!("Type your guess:");

        let mut get_guess = String::new(); // get_guess = ""

        io::stdin()
            .read_line(&mut get_guess)
            .expect("Failed to read line");

        // let get_guess: u32 = get_guess.trim().parse().expect("Type a number!");

        let get_guess: u32 = match get_guess.trim().parse() {
            Ok(num) => num,
            // Err(_) => continue,
            Err(err) => {
                println!("Ignore: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", get_guess);

        match get_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("YOU WIN CONGRATS!");
                break;
            }
        }
    }
}
