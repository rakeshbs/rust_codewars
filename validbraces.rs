fn valid_braces(s: &str) -> bool {
    let opening_braces: Vec<char> = vec!['(', '{', '['];
    let closing_braces: Vec<char> = vec![')', '}', ']'];
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if opening_braces.contains(&c) {
            stack.push(c)
        } else {
            if let Some(top) = stack.pop() {
                let ind1 = closing_braces.iter().position(|&curr| curr == c).unwrap();
                let ind2 = opening_braces.iter().position(|&curr| curr == top).unwrap();

                if ind1 != ind2 {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    if stack.is_empty() {
        return true;
    };
    false
}

fn main() {
    expect_true("()");
    expect_false("(((({{");
}

fn expect_true(s: &str) {
    assert!(
        valid_braces(s),
        "Expected {s:?} to be valid. Got false",
        s = s
    );
}

fn expect_false(s: &str) {
    assert!(
        !valid_braces(s),
        "Expected {s:?} to be invalid. Got true",
        s = s
    );
}
