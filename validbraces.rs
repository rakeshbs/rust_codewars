fn valid_braces(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            x => {
                if Some(x) != stack.pop() {
                    return false;
                }
            }
        }
    }
    return stack.is_empty();
}
fn main() {
    expect_true("()");
    expect_false("}}}");
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
