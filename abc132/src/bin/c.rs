use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n],
    }
    d.sort();
    println!("{}", d[n / 2] - d[n / 2 - 1]);
}
