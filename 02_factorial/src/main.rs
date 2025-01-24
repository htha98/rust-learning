use std::io;

const MAX_INPUT: u8 = 33;

fn factorial(input_number: u8) -> u128 {
    if input_number == 0 {
        1
    } else {
        input_number as u128 * factorial(input_number - 1)
    }
}

fn main() {
    let mut user_input = String::new();

    loop {
        println!(
            "Enter the number (<=32) to calculate factorial (enter `q` to exit the program): "
        );
        user_input.clear();

        if io::stdin().read_line(&mut user_input).is_err() {
            println!("Failed to read line. Try it again.");
            continue;
        };

        if user_input.trim() == "q" {
            break;
        }

        let parsed_user_input: u128 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{user_input} is not number.");
                continue;
            }
        };
        if parsed_user_input > MAX_INPUT.into() {
            println!("Maximum factorial program can calculate is !{MAX_INPUT}.");
            println!("You entered {parsed_user_input}.");
            println!("Try again or force kill the program (Ctrl-C).");
            continue;
        }
        let result: u128 = factorial(parsed_user_input as u8);
        println!("!{parsed_user_input} = {result}");
    }
}
