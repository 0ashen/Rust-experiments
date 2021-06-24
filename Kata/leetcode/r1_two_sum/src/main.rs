// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut was_found: bool = false;
    'outer: for i in 0..(nums.len() as i32) {
        'inner: for y in i..(nums.len() as i32) {
            if nums[i as usize] + nums[y as usize] == target && i != y {
                result.push(i);
                result.push(y);
                was_found = true;
                break 'inner;
            }
        }
        if was_found {
            break 'outer;
        }
    }
    return result
}

fn main() {
    println!("RESULT ================ {}", two_sum(vec![3, 2, 4], 6)[0])
}
