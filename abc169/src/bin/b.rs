use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let res = a.iter().fold(1_i64, |acc, &a| acc.saturating_mul(a));
    if res <= 1e18 as i64 {
        println!("{}", res);
    } else {
        println!("{}", -1);
    }
}
