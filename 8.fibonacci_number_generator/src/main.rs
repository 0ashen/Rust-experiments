use std::io;

fn main() {
    loop {
        println!("Please input count of numbers fibonacci(>2):");

        let mut numbers_count = String::new();

        io::stdin()
            .read_line(&mut numbers_count)
            .expect("Failed to read line");

        let numbers_count: u32 = match numbers_count.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if numbers_count <= 2 {
            continue
        }

        let mut index = 2;
        let mut last_last_num = 1;
        let mut last_num = 1;
        loop {
            println!("Fibonacci: {}", last_num);
            index += 1;
            let temp = last_num;
            last_num = last_num + last_last_num;
            last_last_num = temp;
            if index == numbers_count {
                break
            }
        }

        break
    }
}
