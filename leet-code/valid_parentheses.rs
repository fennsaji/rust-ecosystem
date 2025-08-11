pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if stack.pop() != Some(ch) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

fn main() {
    let test_cases = vec![
        ("()", true),
        ("()[]{}", true),
        ("(]", false),
        ("([)]", false),
        ("{[]}", true),
        ("", true), // Edge case: empty string
    ];

    for (input, expected) in test_cases {
        let result = is_valid(input.to_string());
        println!("Input: \"{}\" | Expected: {} | Got: {} -> {}", input, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}