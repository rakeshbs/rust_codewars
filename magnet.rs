fn calc_force(k: f64, n: f64) -> f64 {
    let inv_force = k * (n + 1.0).powf(2.0 * k);
    return 1.0 / inv_force;
}
fn doubles(maxk: i32, maxn: i32) -> f64 {
    let mut sum = 0.0_f64;
    for k in 1..=maxk {
        for n in 1..=maxn {
            sum += calc_force(k as f64, n as f64);
        }
    }
    return sum;
}

fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
    let res = doubles(maxk, maxn);
    println!("{}", res);
}

fn main() {
    dotest(1, 10, 0.5580321939764581);
    dotest(10, 1000, 0.6921486500921933);
    dotest(10, 10000, 0.6930471674194457);
}
