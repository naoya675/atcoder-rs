use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut res = 0;
    for i in 1..n {
        res += (n - 1) / i;
    }
    println!("{}", res);
}
