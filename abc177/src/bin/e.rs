use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut prime = vec![false; 1 << 20];
    let mut pairwise_coprime = true;
    for &a in &a {
        for (p, _) in prime_factorization(a) {
            if prime[p] {
                pairwise_coprime = false;
            }
            prime[p] = true;
        }
    }
    if pairwise_coprime {
        println!("pairwise coprime");
    } else if a.iter().fold(0, |acc, &a| gcd(acc, a)) == 1 {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
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
