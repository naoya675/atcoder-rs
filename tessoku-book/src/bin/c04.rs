use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    for i in enumerate_divisors(n) {
        println!("{}", i);
    }
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
