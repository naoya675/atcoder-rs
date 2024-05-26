use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }
    let m = 998244353;
    let mut power = vec![1_i64; n + 1];
    for i in 0..n {
        power[i + 1] = (100 * power[i]) % m;
    }
    let mut hash1 = vec![0_i64; n + 1];
    let mut hash2 = vec![0_i64; n + 1];
    for i in 0..n {
        hash1[i + 1] = (100 * hash1[i] + s[i] as i64) % m;
        hash2[i + 1] = (100 * hash2[i] + s[n - i - 1] as i64) % m;
    }
    for (l, r) in lr {
        let mut hash1 = hash1[r] - (power[r - l + 1] * hash1[l - 1]) % m;
        let l_rev = n + 1 - r;
        let r_rev = n + 1 - l;
        let mut hash2 = hash2[r_rev] - (power[r_rev - l_rev + 1] * hash2[l_rev - 1]) % m;
        if hash1 < 0 {
            hash1 += m;
        }
        if hash2 < 0 {
            hash2 += m;
        }
        println!("{}", if hash1 == hash2 { "Yes" } else { "No" });
    }
}
