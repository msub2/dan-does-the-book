use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush!");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Not a number.");
    
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("nope");
            return;
        }
    };

    println!("{}", fibonacci(number));
}

fn fibonacci(number: u32) -> u32 {
    if number <= 1 {
        return number;
    } else {
        return fibonacci(number - 1) + fibonacci(number - 2);
    }
}