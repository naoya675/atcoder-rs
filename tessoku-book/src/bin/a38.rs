use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, i32); n],
    }
    let mut dp = vec![24; d];
    for (l, r, h) in lrh {
        for i in l..=r {
            dp[i - 1] = dp[i - 1].min(h);
        }
    }
    println!("{}", dp.iter().sum::<i32>());
}
