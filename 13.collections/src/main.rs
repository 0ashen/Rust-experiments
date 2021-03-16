// fn main() {
//     let v: Vec<i32> = Vec::new();
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//
//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);
//
//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
// }
// fn main() {
//     let v = vec![1, 2, 3];
// }
// fn main() {
//     let mut v = Vec::new();
//
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
// }

// fn main() {
//     let data = "initial contents";
//
//     let s = data.to_string();
//
//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
// }

// fn main() {
//     let hello = String::from("السلام عليكم");
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("שָׁלוֹם");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     let hello = String::from("Hola");
// }

// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// }

// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);
//
//     println!("{:?}", scores);
// }

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
