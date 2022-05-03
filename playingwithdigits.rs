fn dig_pow(n: i64, p: i32) -> i64 {
    let digits: Vec<i64> = n.to_string()
        .chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
    let mut k = p as u32;
    let mut sum : i64 = 0;
    for d in digits {
        sum += d.pow(k) as i64;
        k += 1;
        println!("{}", sum);
    }
    if sum % n != 0 { return -1;}
    return  sum / n;
}

fn dotest(n: i64, p: i32, exp: i64) -> () {
    //println!(" n: {:?};", n);
    //println!("p: {:?};", p);
    let ans = dig_pow(n, p);
    //println!(" actual:\n{:?};", ans);
    //println!("expect:\n{:?};", exp);
    //println!(" {};", ans == exp);
    //assert_eq!(ans, exp);
    //println!("{};", "-");
}

fn main() {
    //dotest(89, 1, 1);
    //dotest(92, 1, -1);
    //dotest(46288, 3, 51);
    dotest(3456789, 5, 51);
}
