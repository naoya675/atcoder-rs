use proconio::input;

fn main() {
    input! {
        mut a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    a -= 1;
    let max = (b / c) + (b / d) - (b / lcm(c, d));
    let min = (a / c) + (a / d) - (a / lcm(c, d));
    println!("{}", (b - a) - (max - min));
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
    a * b / gcd(a, b)
}
