use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [i32; n - 1],
    }
    let mut a = vec![1 << 30; n];
    for i in 0..n - 1 {
        a[i] = a[i].min(b[i]);
        a[i + 1] = a[i + 1].min(b[i]);
    }
    println!("{}", a.iter().sum::<i32>());
}
