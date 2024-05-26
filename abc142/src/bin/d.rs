use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", prime_factorization(gcd(a, b)).len() + 1);
}

fn gcd(a: i64, b: i64) -> i64 {
    assert!(a > 0);
    assert!(b > 0);
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
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
