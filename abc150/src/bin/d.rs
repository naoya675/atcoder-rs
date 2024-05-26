use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }
    let a = a.iter().map(|a| a / 2).collect::<Vec<_>>();
    let l = a.iter().fold(1, |acc, &a| lcm(acc, a));
    if a.iter().all(|&a| (l / a) % 2 != 0) {
        println!("{}", (m / l + 1) / 2);
    } else {
        println!("{}", 0);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    assert!(a > 0);
    assert!(b > 0);
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)).saturating_mul(b)
}
