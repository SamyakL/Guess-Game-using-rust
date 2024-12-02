use rand::Rng;
use std::io;
fn main() {
    let fruits: [&str; 5] = ["apple", "mango", "grapes", "watermelon", "pineapple"];

    println!("WelCome!");

    println!("Choose any one of the follwoing fruits:{:?} ", fruits);

    let computer_guess: usize = guess_number();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("inputFalied!");
    user_input = user_input.trim().to_string();
    println!(
        "Here is your input:{user_input} and its length is : {} ",
        user_input.len()
    );

    let computer_guessed_fruit = fruits[computer_guess];
    println!(
        "Here is what computer has guessed:{} ",
        computer_guessed_fruit
    );

    if user_input == computer_guessed_fruit {
        println!("You guessed it right!, Congratulations!");
        return;
    } else {
        println!("Seems ur guess was wrong, try again later!");
    }
}

fn guess_number() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..5);
}
