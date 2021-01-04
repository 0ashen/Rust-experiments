use std::io;

fn main() {
    loop {
        println!("Please input temperature in Fahrenheit:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let mut guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        guess = (guess - 32) * 5 / 9;
        println!("Your temperature in Fahrenheit: {}", guess);
        break
    }
}
