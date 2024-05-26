use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i32; n],
    }
    a.sort();
    println!("{}", a[..k].iter().sum::<i32>());
}
