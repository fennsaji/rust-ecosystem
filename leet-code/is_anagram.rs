// Given two strings s and t, return true if the two strings are anagrams of each other, otherwise return false.
// An anagram is a string that contains the exact same characters as another string, but the order of the charaters can be different.
// Example 1:
// Input: s = "racecar", t = "carrace"
// Output: true
// Example 2:
// Input: s = "jar", t = "jam"
// Output: false
// Constraints:
// s and t consist of lowercase English letters.
use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();
    for a in s.chars() {
        *s_map.entry(a).or_insert(0) += 1;
    }
    for a in t.chars() {
        *t_map.entry(a).or_insert(0) += 1;
    }
    for (key, value) in s_map {
        if t_map.get(&key) != Some(&value) {
            return false;
        }
    }
    true
}

fn main() {
    let s = String::from("racecar");
    let t = String::from("carrace");
    let result = is_anagram(s, t);
    println!("Are the strings anagrams? {}", result); // Should print: Are the strings anagrams? true
}