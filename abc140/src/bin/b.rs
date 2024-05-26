use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [i32; n],
        c: [i32; n - 1],
    }
    let mut res = b.iter().sum::<i32>();
    for i in 0..n - 1 {
        if a[i] == a[i + 1] - 1 {
            res += c[a[i] - 1];
        }
    }
    println!("{}", res);
}
