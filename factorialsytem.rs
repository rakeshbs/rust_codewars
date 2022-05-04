fn dec2_fact_string(nb: u64) -> String {
    let mut factstring: String = String::from("");
    let mut n = nb;
    let mut f = 1;
    while n > 0 {
        let rem = n % f;
        let c = std::char::from_digit(rem as u32, 36).unwrap();
        let numeral = format!("{}", c).to_uppercase();
        factstring = format!("{}{}", numeral, factstring);
        n /= f;
        f += 1;
    }
    return factstring;
}

fn fact_string_2dec(s: String) -> u64 {
    let mut res: u64 = 0;
    let mut fact = 1;
    let mut counter = 1;
    for c in s.chars().rev() {
        let d = c.to_digit(36).unwrap() as u64;
        res += d * fact;
        fact *= counter;
        counter += 1;
    }
    return res;
}

fn testing1(nb: u64, exp: &str) -> () {
    assert_eq!(&dec2_fact_string(nb), exp)
}

fn testing2(s: &str, exp: u64) -> () {
    assert_eq!(fact_string_2dec(s.to_string()), exp)
}

fn basics_dec2_fact_string() {
    testing1(2982, "4041000");
    testing1(463, "341010");
}
fn basics_fact_string_2dec() {
    testing2("4041000", 2982);
    testing2("341010", 463);
}

fn main() {
    basics_dec2_fact_string();
    basics_fact_string_2dec();
}
