// fn main() {
//     panic!("crash and burn");
// }



// #![allow(unused)]
// fn main() {
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }


// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
// }



#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
}
