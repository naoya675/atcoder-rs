use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut res = 0;
    for (_, c) in prime_factorization(n) {
        for _ in (1..).take_while(|&i| i * (i + 1) / 2 <= c) {
            res += 1;
        }
    }
    println!("{}", res);
}

fn prime_factorization(mut n: i64) -> Vec<(i64, usize)> {
    let mut res = vec![];
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut e = 0;
        while n % i == 0 {
            n /= i;
            e += 1;
        }
        if e != 0 {
            res.push((i, e));
        }
    }
    if n != 1 {
        res.push((n, 1));
    }
    res.sort();
    res
}
