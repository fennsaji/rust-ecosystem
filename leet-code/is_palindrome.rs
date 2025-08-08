pub fn is_palindrome(s: String) -> bool {
    let s_cleared = s
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect::<String>();
    let s_rev: String = s_cleared.chars().rev().collect();
    s_cleared == s_rev
}

pub fn is_palindrome_v2(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut l = 0;
    let mut r = bytes.len().saturating_sub(1);

    while l < r {
        while l < r && !is_alphanumeric(bytes[l]) {
            l += 1;
        }
        while r > l && !is_alphanumeric(bytes[r]) {
            r -= 1;
        }

        if l < r && to_lower(bytes[l]) != to_lower(bytes[r]) {
            return false;
        }

        l += 1;
        r = r.saturating_sub(1);
    }

    true
}

fn is_alphanumeric(c: u8) -> bool {
    (b'A' <= c && c <= b'Z') ||
    (b'a' <= c && c <= b'z') ||
    (b'0' <= c && c <= b'9')
}

fn to_lower(c: u8) -> u8 {
    if b'A' <= c && c <= b'Z' {
        c + 32
    } else {
        c
    }
}


fn main() {
    let test_cases = vec![
        ("A man, a plan, a canal: Panama", true),
        ("race a car", false),
        (" ", true),
    ];

    for (input, expected) in test_cases {
        let result = is_palindrome_v2(input.to_string());
        println!("Input: \"{}\" | Expected: {} | Got: {} -> {}", input, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}