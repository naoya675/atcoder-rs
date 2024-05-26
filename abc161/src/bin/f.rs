use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut res = enumerate_divisors(n - 1).len() - 1;
    for i in enumerate_divisors(n) {
        if i == 1 {
            continue;
        }
        let mut n = n;
        while n % i == 0 {
            n /= i;
        }
        if n % i == 1 {
            res += 1;
        }
    }
    println!("{}", res);
}

fn enumerate_divisors(n: i64) -> Vec<i64> {
    let mut res = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res.push(i);
            if n / i != i {
                res.push(n / i);
            }
        }
    }
    res.sort();
    res
}
