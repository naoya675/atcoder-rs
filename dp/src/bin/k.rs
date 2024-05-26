use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![false; k + 1];
    for i in 0..k {
        for a in a.iter() {
            if i + a <= k {
                dp[i + a] |= !dp[i];
            }
        }
    }
    println!("{}", if dp[k] { "First" } else { "Second" });
}
