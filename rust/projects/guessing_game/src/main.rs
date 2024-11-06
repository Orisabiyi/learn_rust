// import standard input and output
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // declaring a mutable variable
    let mut guess = String::new();

    // generating random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");


    io::stdin()
    .read_line(&mut guess)
    .expect("Faild to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    // comparing
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}