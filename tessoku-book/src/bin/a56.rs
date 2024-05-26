use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        abcd: [(usize, usize, usize, usize); q],
    }
    let m = 998244353;
    let mut power = vec![1_i64; n + 1];
    for i in 0..n {
        power[i + 1] = (100 * power[i]) % m;
    }
    let mut hash = vec![0_i64; n + 1];
    for i in 0..n {
        hash[i + 1] = (100 * hash[i] + s[i] as i64) % m;
    }
    for (a, b, c, d) in abcd {
        let mut hash1 = hash[b] - (power[b - a + 1] * hash[a - 1]) % m;
        let mut hash2 = hash[d] - (power[d - c + 1] * hash[c - 1]) % m;
        if hash1 < 0 {
            hash1 += m;
        }
        if hash2 < 0 {
            hash2 += m;
        }
        println!("{}", if hash1 == hash2 { "Yes" } else { "No" });
    }
}
