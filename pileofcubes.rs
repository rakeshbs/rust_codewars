fn find_nb(m: u64) -> i32 {
    let mut i: u64 = 1;
    while i * i < m {
        let mut s = i * (i + 1) / 2;
        s = s * s;
        if s == m {
            return i as i32;
        } else if s > m {
            return -1;
        }
        i += 1;
    }
    return -1;
}

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
}

fn main() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
