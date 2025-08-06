pub fn is_palindrome(s: String) -> bool {
    let s_cleared = s
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect::<String>();
    let s_rev: String = s_cleared.chars().rev().collect();
    s_cleared == s_rev
}

fn main() {
    let test_cases = vec![
        ("A man, a plan, a canal: Panama", true),
        ("race a car", false),
        (" ", true),
    ];

    for (input, expected) in test_cases {
        let result = is_palindrome(input.to_string());
        println!("Input: \"{}\" | Expected: {} | Got: {} -> {}", input, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}