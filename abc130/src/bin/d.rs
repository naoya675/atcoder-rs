use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut res = 0;
    let mut i = 0;
    for j in 0..n {
        while i <= n && acc[i] - acc[j] < k {
            i += 1;
        }
        res += (n + 1) - i;
    }
    println!("{}", res);
}
