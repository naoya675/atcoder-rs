use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    let mut pre = vec![0; n + 2];
    let mut suf = vec![0; n + 2];
    for i in 1..=n {
        pre[i] = pre[i - 1].max(a[i - 1]);
    }
    for i in (1..=n).rev() {
        suf[i] = suf[i + 1].max(a[i - 1]);
    }
    for (l, r) in lr {
        println!("{}", pre[l - 1].max(suf[r + 1]));
    }
}
