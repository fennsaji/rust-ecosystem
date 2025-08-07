pub fn min_start_value(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 1;
    }
    let mut new_vec: Vec<i32> = vec![];
    let mut smallest_value = 0;
    for (i,n) in nums.iter().enumerate() {
        if i <= 0 {
            new_vec.push(*n);
        } else {
            new_vec.push(new_vec[i - 1] + n);
        }
        if new_vec[i] < smallest_value {
            smallest_value = new_vec[i];
        }
    }
    smallest_value.abs() + 1
}

fn main() {
    let nums = vec![1, 2, -3];
    let result = min_start_value(nums);
    println!("Minimum start value: {}", result); // Should print: Minimum start value: 3
    let nums2 = vec![1, 2, -5, 3];
    let result2 = min_start_value(nums2);
    println!("Minimum start value: {}", result2); // Should print: Minimum start value:
    let nums3 = vec![-1, -2, -3];
    let result3 = min_start_value(nums3);
    println!("Minimum start value: {}", result3); // Should print: Minimum start value:
}
