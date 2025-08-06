pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut current_index: i32 = -1;
    if needle.is_empty() {
        return 0;
    }
    if haystack.is_empty() || haystack.len() < needle.len() {
        return -1;
    }

    for i in 0..haystack.len() {
        if let Some(h) = haystack.chars().nth(i) {
            if let Some(n) = needle.chars().nth(0) {
                if h == n {
                    current_index = i as i32;
                    for j in 0..needle.len() {
                        if let Some(h2) = haystack.chars().nth(i + j) {
                            if let Some(n2) = needle.chars().nth(j) {
                                if h2 != n2 {
                                    current_index = -1;
                                    break;
                                }
                            }
                        } else {
                            current_index = -1;
                            break;
                        }
                    }
                    if current_index != -1 {
                        break
                    }
                }
            }
        }
    }
    current_index
}

pub fn str_str_v2(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }

    let hay = haystack.as_bytes();
    let nee = needle.as_bytes();
    let h_len = hay.len();
    let n_len = nee.len();

    if n_len > h_len {
        return -1;
    }

    for i in 0..=h_len - n_len {
        if &hay[i..i + n_len] == nee {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let test_cases = vec![
        ("hello", "ll", 2),
        ("aaaaa", "bba", -1),
        ("mississippi", "issipi", -1),
        ("abc", "c", 2),
        ("abcde", "f", -1),
    ];

    for (haystack, needle, expected) in test_cases {
        let result = str_str_v2(haystack.to_string(), needle.to_string());
        println!("Haystack: \"{}\", Needle: \"{}\" | Expected: {} | Got: {} -> {}", haystack, needle, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}