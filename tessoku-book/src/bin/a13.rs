use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }
    let mut res = 0;
    let mut j = 0;
    for i in 0..n {
        while j + 1 < n && a[j + 1] - a[i] <= k {
            j += 1;
        }
        res += j - i;
    }
    println!("{}", res);
}
