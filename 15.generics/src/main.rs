// #[allow(dead_code)]
// fn largest2(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
// #[allow(dead_code)]
// fn main2() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(result, 100);
//
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(result, 6000);
// }
//
// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main3() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(result, 100);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
//     assert_eq!(result, 'y');
// }
// #[allow(dead_code)]
// fn main1() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let mut largest = number_list[0];
//
//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     println!("The largest number is {}", largest);
//     assert_eq!(largest, 100);
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     // let error = Point { x: 1.0, y: 4 };
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
