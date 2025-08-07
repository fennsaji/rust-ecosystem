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

 pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut second_s = t.clone();
    for a in s.chars() {
        second_s = second_s.replacen(a, "", 1);
    }
    if second_s.is_empty() {
        return true;
    }
    false
}

fn main() {
    let s = String::from("racecar");
    let t = String::from("carrace");
    let result = is_anagram(s, t);
    println!("Are the strings anagrams? {}", result); // Should print: Are the strings anagrams? true
}