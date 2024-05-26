use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let s = s
        .iter()
        .map(|s| s.to_digit(10).unwrap() as usize)
        .rev()
        .collect::<Vec<_>>();
    let mut count = vec![0; 2019];
    let mut num = 0;
    count[num] += 1;
    for (i, s) in s.iter().enumerate() {
        num = (num + s * power(10, i, 2019)) % 2019;
        count[num] += 1;
    }
    let mut res = 0;
    for c in count {
        res += c * (c - 1) / 2;
    }
    println!("{}", res);
}

fn power(a: usize, b: usize, m: usize) -> usize {
    let mut res = 1;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b % 2 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res
}
