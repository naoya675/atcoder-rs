use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for s in &s {
        match s {
            'R' => r += 1,
            'G' => g += 1,
            'B' => b += 1,
            _ => unreachable!(),
        }
    }
    let mut res: i64 = r * g * b;
    for i in 0..n {
        for j in i..n {
            let k = j + (j - i);
            if k < n && s[i] != s[j] && s[j] != s[k] && s[k] != s[i] {
                res -= 1;
            }
        }
    }
    println!("{}", res);
}
