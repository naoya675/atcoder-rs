use proconio::input;

fn main() {
    input! {
        mut n: i32,
    }
    while !is_prime(n) {
        n += 1;
    }
    println!("{}", n);
}

fn is_prime(n: i32) -> bool {
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}
