use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let mut res = vec![];
    for i in 0..n {
        res.push(s[i]);
        res.push(t[i]);
    }
    println!("{}", res.iter().collect::<String>());
}
