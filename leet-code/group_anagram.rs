// Given an array of strings strs, group all anagrams together into sublists. You may return the output in any order.
// An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.
// Example 1:
// Input: strs = ["act","pots","tops","cat","stop","hat"]
// Output: [["hat"],["act", "cat"],["stop", "pots", "tops"]]
// Example 2:
// Input: strs = ["x"]
// Output: [["x"]]
// Example 3:
// Input: strs = [""]
// Output: [[""]]
// Constraints:
// 1 <= strs.length <= 1000.
// 0 <= strs[i].length <= 100
// strs[i] is made up of lowercase English letters.

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut count = [0; 26];
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        map.entry(count).or_insert(vec![]).push(s);
    }

    map.into_values().collect()
}

fn main() {
    let strs = vec!["act".to_string(), "pots".to_string(), "tops".to_string(), "cat".to_string(), "stop".to_string(), "hat".to_string()];
    let result = group_anagrams(strs);
    println!("Grouped anagrams: {:?}", result); // Should print: Grouped anagrams: [["hat"], ["act", "cat"], ["stop", "pots", "tops"]]
    
    let strs2 = vec!["x".to_string()];
    let result2 = group_anagrams(strs2);
    println!("Grouped anagrams: {:?}", result2); // Should print: Grouped anagrams: [["x"]]
    
    let strs3 = vec!["".to_string()];
    let result3 = group_anagrams(strs3);
    println!("Grouped anagrams: {:?}", result3); // Should print: Grouped anagrams: [[""]]
}