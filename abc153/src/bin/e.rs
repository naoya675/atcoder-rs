use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(usize, i32); n],
    }
    let mut dp = vec![1 << 30; h + 1];
    dp[0] = 0;
    for (a, b) in ab {
        for j in 0..h {
            dp[(j + a).min(h)] = dp[(j + a).min(h)].min(dp[j] + b);
        }
    }
    println!("{}", dp[h]);
}
