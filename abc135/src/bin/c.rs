use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n+1],
        b: [i64; n],
    }
    let mut res = 0;
    for i in 0..n {
        if b[i] > a[i] {
            res += a[i];
            res += a[i + 1].min(b[i] - a[i]);
            a[i + 1] -= a[i + 1].min(b[i] - a[i]);
        } else {
            res += b[i];
        }
    }
    println!("{}", res);
}
