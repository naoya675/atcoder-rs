use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [i32; q],
    }
    for i in 0..q {
        println!("{}", if is_prime(x[i]) { "Yes" } else { "No" });
    }
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
