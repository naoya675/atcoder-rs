use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let prime = sieve_of_eratosthenes(n);
    for p in prime {
        println!("{}", p);
    }
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut prime = vec![];
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            prime.push(i);
            let mut j = i + i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    prime
}
