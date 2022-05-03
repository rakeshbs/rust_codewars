fn solution(num: i32) -> i32 {
    let mut sum : i32 = 0;
    let mut multiple : i32 = 0;
    while multiple < num {
        sum += multiple;
        multiple += 3;
    }
    multiple = 0;
    while multiple < num {
        if multiple % 3 != 0 { sum += multiple; }
        multiple += 5;
    }
    return sum;
}

fn main() {
  assert_eq!(solution(10), 23);
  assert_eq!(solution(11), 33);
  assert_eq!(solution(6), 8);
}
