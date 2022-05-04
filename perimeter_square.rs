fn perimeter(n: u64) -> u64 {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut sum = 1;
    for _ in 0..n {
        let n3 = n1 + n2;
        n1 = n2;
        n2 = n3;
        sum += n3;
    }
    return sum * 4;
}

fn dotest(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

fn main() {
    dotest(5, 80);
    dotest(7, 216);
    dotest(20, 114624);
    dotest(30, 14098308);
}
