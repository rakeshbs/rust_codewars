fn ips_between(start: &str, end: &str) -> u32 {
    let start_split: Vec<&str> = start.split('.').collect();
    let end_split: Vec<&str> = end.split('.').collect();
    let mut total: i64 = 0;
    for i in (0..4).rev() {
        let st: i64 = start_split[i].parse().unwrap();
        let en: i64 = end_split[i].parse().unwrap();
        let diff = 256_i64.pow(3 - (i as u32)) * (en - st);
        total += diff;
    }
    return total as u32;
}

fn main() {
    assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
    assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
}
