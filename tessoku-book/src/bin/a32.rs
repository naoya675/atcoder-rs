use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut dp = vec![false; n + 1];
    for i in 0..n {
        if i + a <= n && dp[i] == false {
            dp[i + a] = true;
        }
        if i + b <= n && dp[i] == false {
            dp[i + b] = true;
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" });
}
