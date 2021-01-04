// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result is {}", result);
// }
//
// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{}!", number);
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//
//     while index < 5 {
//         println!("the value is: {}", a[index]);
//
//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
