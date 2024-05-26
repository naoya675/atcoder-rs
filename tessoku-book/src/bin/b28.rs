use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let m = 1_000_000_007;
    let mut fibonacci = vec![0; n];
    fibonacci[0] = 1;
    fibonacci[1] = 1;
    for i in 2..n {
        fibonacci[i] = (fibonacci[i - 1] + fibonacci[i - 2]) % m;
    }
    println!("{}", fibonacci[n - 1]);
}
