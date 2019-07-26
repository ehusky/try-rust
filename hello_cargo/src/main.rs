use std::io;

fn in_output() {
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

fn main() {
    println!("Hello, world!");
    in_output();
    return();
}
