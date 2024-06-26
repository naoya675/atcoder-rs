use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", lcm(a, b));
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
