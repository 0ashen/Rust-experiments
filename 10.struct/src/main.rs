// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//
// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//
//     user1.email = String::from("anotheremail@example.com");
// }

/*struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
*/

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
//
// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//
// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//
//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..user1
//     };
// }
// fn TupleStructs() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
//
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }
//

// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/*fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
