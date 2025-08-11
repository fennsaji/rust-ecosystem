pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2; // Solves Integer Oerflow issue
        
        if nums[mid] == target {
            return mid as i32;
        }
        
        if nums[mid] > target {
            if mid == 0 { break; }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    
    -1
}

fn main() {
    let test_cases = vec![
        (vec![0, 1, 2, 4, 5, 6, 7], 0, 0),
        (vec![0, 1, 2, 4, 5, 6, 7], 3, -1),
        (vec![], 0, -1), // Edge case: empty array
        (vec![1], 1, 0), // Edge case: single element found
        (vec![1], 2, -1), // Edge case: single element not found
        (vec![2, 5], 5, 1), // Edge case: two elements, target found
        (vec![2, 5], 3, -1), // Edge case: two elements, target not found
    ];

    for (nums, target, expected) in test_cases {
        let result = search(nums.clone(), target);
        println!("Input: {:?}, Target: {} | Expected: {} | Got: {} -> {}", nums, target, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}
