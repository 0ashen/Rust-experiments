// solution kata https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust
fn main(num: i32) -> i32 {
    if num <= 0 { return 0; };

    let mut result = 0;

    for x in 0..num {
        if x % 3 == 0 || x % 5 == 0 {
            result += x;
        }
    }

    result
}