use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let res = (n / 3) + (n / 5) - (n / 15);
    println!("{}", res);
}
