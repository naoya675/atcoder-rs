use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut res = n;
    for i in (1..).take_while(|&i| i * i <= n) {
        if n % i == 0 {
            res = res.min(i + n / i - 2);
        }
    }
    println!("{}", res);
}
