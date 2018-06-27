
use std::io;
use std::io::Write;

fn main() {

    loop {
        print!("Farenheit temperature: ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line!");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input is not a valid temperature");
                continue
            },
        };

        let celsius = (temp - 32.0) * (5.0 / 9.0);

        println!("{} Farenheit = {:.2} Celsius", temp, celsius);
        break;
    }
}
