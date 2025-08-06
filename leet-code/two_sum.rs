use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        
        map.insert(num, i);
    }
    
    vec![]
}

fn main() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ];

    for (input, target, expected) in test_cases {
        let result = two_sum(input.clone(), target);
        println!("Input: {:?}, Target: {} | Expected: {:?} | Got: {:?} -> {}", input, target, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_different_order() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_same_number() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}