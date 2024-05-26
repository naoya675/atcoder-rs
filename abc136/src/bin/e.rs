use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let sum = a.iter().sum::<i64>();
    let mut res = 0;
    for d in enumerate_divisors(sum) {
        let mut a = a.iter().map(|a| a % d).collect::<Vec<_>>();
        a.sort();
        let mut inc_vec = vec![0; n + 1];
        let mut dec_vec = vec![0; n + 1];
        for i in 0..n {
            inc_vec[i + 1] = inc_vec[i] + a[i];
            dec_vec[i + 1] = dec_vec[i] + (d - a[i]);
        }
        for i in 0..n + 1 {
            let inc = inc_vec[i] - inc_vec[0];
            let dec = dec_vec[n] - dec_vec[i];
            if inc <= k && inc == dec {
                res = res.max(d);
            }
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
