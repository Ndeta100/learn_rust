use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..100);
    println!("the secret number is {}", secret_number);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed {}", guess);
    let guess = guess.trim().parse();
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big"),
    }
}
