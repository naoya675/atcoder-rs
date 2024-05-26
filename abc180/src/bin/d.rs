use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        a: i64,
        b: i64,
    }
    let mut str = x;
    let mut res = 0;
    while str.saturating_mul(a) < str.saturating_add(b) && str.saturating_mul(a) < y {
        str *= a;
        res += 1;
    }
    res += (y - str - 1) / b;
    println!("{}", res);
}
