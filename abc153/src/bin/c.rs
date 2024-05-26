use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [i64; n],
    }
    h.sort();
    println!("{}", h[..n - n.min(k)].iter().sum::<i64>());
}
