use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut vec = vec![0; n + 1];
    for i in 0..n {
        vec[i + 1] = vec[i] + a[i];
    }
    let mut res = 0;
    let mut j = 0;
    for i in 0..n {
        while j < n && vec[j + 1] - vec[i] <= k {
            j += 1;
        }
        res += j - i;
    }
    println!("{}", res);
}
