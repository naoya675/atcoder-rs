use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let n = n
        .iter()
        .map(|n| n.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let mut dp0 = vec![1 << 60; n.len() + 1];
    let mut dp1 = vec![1 << 60; n.len() + 1];
    dp0[0] = 0;
    for i in 0..n.len() {
        dp0[i + 1] = dp0[i + 1].min(dp0[i] + n[i]);
        dp0[i + 1] = dp0[i + 1].min(dp1[i] + (n[i] + 1));
        dp1[i + 1] = dp1[i + 1].min(dp0[i] + 10 - n[i]);
        dp1[i + 1] = dp1[i + 1].min(dp1[i] + 10 - (n[i] + 1));
    }
    println!("{}", dp0[n.len()].min(dp1[n.len()] + 1));
}
