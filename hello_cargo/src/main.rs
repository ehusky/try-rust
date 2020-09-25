use std::io;

fn guess_num() {
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

fn main() {
    println!("Hello, world!");
    guess_num();
    return();
}
