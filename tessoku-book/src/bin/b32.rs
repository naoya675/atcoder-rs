use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }
    let mut dp = vec![false; n + 1];
    for i in 0..n {
        for a in &a {
            if i + a <= n && dp[i] == false {
                dp[i + a] = true;
            }
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" });
}
