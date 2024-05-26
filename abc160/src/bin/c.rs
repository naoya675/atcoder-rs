use proconio::input;

fn main() {
    input! {
        k: i64,
        n: usize,
        mut a: [i64; n],
    }
    a.push(a[0] + k);
    println!("{}", k - a.windows(2).map(|a| a[1] - a[0]).max().unwrap());
}
