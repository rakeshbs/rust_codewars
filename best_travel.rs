fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let mut visited: Vec<bool> = vec![false; ls.len()];
    visited.fill(false);
    let mut best = 1000000;
    let best = choose(t, k, ls, &mut visited);
    if best >= 1000000 {
        return -1;
    }
    return t - best;
}

fn choose(t: i32, k: i32, ls: &Vec<i32>, visited: &mut Vec<bool>) -> i32 {
    if t < 0 {
        return 1000000;
    }
    if k <= 0 {
        return t;
    }

    let mut min = 10000000;
    for i in 0..ls.len() {
        if !visited[i] {
            visited[i] = true;
            let c = choose(t - ls[i], k - 1, ls, visited);
            if c < min {
                min = c
            }
            visited[i] = false;
        }
    }
    return min;
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

fn main() {
    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);
}
