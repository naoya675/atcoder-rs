use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        p: usize,
        s: Chars,
    }
    let s = s
        .iter()
        .map(|&s| s as usize - '0' as usize)
        .collect::<Vec<_>>();
    let res = match 10 % p {
        0 => {
            let mut res = 0;
            for i in 0..n {
                if s[i] % p == 0 {
                    res += i + 1;
                }
            }
            res
        }
        _ => {
            let mut res = 0;
            let mut acc = 0;
            let mut count = vec![0; p];
            count[0] = 1;
            for i in (0..n).rev() {
                acc = (acc + s[i] * power(10, n - i - 1, p)) % p;
                res += count[acc];
                count[acc] += 1;
            }
            res
        }
    };
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
