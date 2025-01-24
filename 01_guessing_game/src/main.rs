use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!"); // macros are denoted with `!`
                                   // `let` statement creates the variable
                                   // variables are by default immutable
                                   // function `rng` returns struct `ThreadRng` providing method `random_range`
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");
        // if `mut` precedes variable name, variable is mutable
        let mut guess = String::new();

        // function `stdin` returns struct `Stdin` providing method `read_line`
        io::stdin()
            // `&` indicates that an argument is a reference
            .read_line(&mut guess)
            // `read_line` returns `Result` value, which is enum of values `Ok` and `Err`
            // an instance of `Result` has an `expect` method returning the value that `Ok` is
            // holding or it will crash the program and display the message passed as argument to
            // the method `read_line` if the value of `Result` is `Err`
            .expect("Failed to read line");

        // original variable `guess` of type `String` is shadowed with variable of type `u32`
        // method `trim` returns itself (`String`) with leading and trailing whitespaces removed
        // method `parse` parses current string into another type and returns `Result`
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
