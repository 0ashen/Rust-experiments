use std::io;

fn main() {
    loop {
        println!("Please input temperature in Fahrenheit:");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        let mut temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        temperature = (temperature - 32) * 5 / 9;
        println!("Your temperature in Fahrenheit: {}", temperature);
        break
    }
}
