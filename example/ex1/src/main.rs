fn main() {
    let mut name = String::from( "asdf");

    change(&mut name);

    println!("{}", name);
}


fn change(text:&mut String) {
    text.push_str("*************")
}