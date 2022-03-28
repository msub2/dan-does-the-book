use std::io;

fn main() {
    loop {
        println!("Select one of the following options:");
        println!("1. Convert Celsius to Fahrenheit");
        println!("2. Convert Fahrenheit to Celsius");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid choice.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => println!("{:.1} F", c_to_f(get_temp())),
            2 => println!("{:.1} C", f_to_c(get_temp())),
            3 => break,
            _ => continue,
        }
    }
}

fn get_temp() -> f64 {
    loop {
        let mut temp = String::new();
        println!("Enter your temperature.");

        io::stdin()
            .read_line(&mut temp)
            .expect("Not a valid temperature.");

        match temp.trim().parse() {
            Ok(num) => {
                let temp = num;
                return temp;
            }
            Err(_) => {
                println!("Not a valid temperature.");
                continue;
            }
        };
    }
}

fn c_to_f(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}

fn f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}
