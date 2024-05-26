use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", gcd(a, b));
}

fn gcd(a: i32, b: i32) -> i32 {
    assert!(a > 0);
    assert!(b > 0);
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}
