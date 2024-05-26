use superslice::Ext;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let s = s
        .iter()
        .map(|&s| s as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let t = t
        .iter()
        .map(|&t| t as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let mut vec = vec![vec![]; 26];
    for (i, &c) in s.iter().enumerate() {
        vec[c].push(i);
    }
    for (i, &c) in s.iter().enumerate() {
        vec[c].push(i + s.len());
    }
    let mut res = 0;
    let mut j = 0;
    for &c in t.iter() {
        if vec[c].len() == 0 {
            println!("-1");
            return;
        }
        let k = vec[c].lower_bound(&j);
        if vec[c][k] + 1 < s.len() {
            j = vec[c][k] + 1;
        } else {
            j = vec[c][k] + 1 - s.len();
            res += 1;
        }
    }
    println!("{}", j + res * s.len());
}
