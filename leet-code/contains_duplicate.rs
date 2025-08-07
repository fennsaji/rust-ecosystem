/// Given an integer array nums, return true if any value appears more than once in the array, otherwise return false.
/// Example 1:
/// Input: nums = [1, 2, 3, 3]
/// Output: true
/// Example 2:
/// Input: nums = [1, 2, 3, 4]
/// Output: false


use std::collections::HashMap;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();;
    for n in nums {
        if let Some(val) = map.get(&n) {
            return true;
        }
        map.insert(n, true);
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 1];
    let result = contains_duplicate(nums);
    println!("Contains duplicate: {}", result); // Should print: Contains duplicate: true
}
