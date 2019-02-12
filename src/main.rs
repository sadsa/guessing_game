use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = generate_secret_number(1, 101);
    println!("The secret number is {}", secret_number);
    init_guessing_sequence(secret_number);
}

fn generate_secret_number(low: i32, high: i32) -> i32 {
    return rand::thread_rng().gen_range(low, high);
}

fn init_guessing_sequence(secret_number: i32) {
    loop {
        println!("Please input your guess.");

        let guess = prompt_for_guess();

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Great guess! You win!");
                break;
            }
        }
    }
}

fn prompt_for_guess() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess;
}
