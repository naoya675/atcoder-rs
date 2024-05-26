use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut pre = vec![0; n + 1];
    let mut suf = vec![0; n + 1];
    for i in 0..n {
        pre[i + 1] = pre[i].max(a[i]);
    }
    for i in (0..n).rev() {
        suf[i] = suf[i + 1].max(a[i]);
    }
    for i in 0..n {
        println!("{}", pre[i].max(suf[i + 1]));
    }
}
