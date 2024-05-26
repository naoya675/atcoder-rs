use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    println!("{}", a[n - 1] + a[n - 2]);
}
