use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut res = 0;
    for i in 0..n {
        if p[i] != i + 1 {
            res += 1;
        }
    }
    println!("{}", if res <= 2 { "YES" } else { "NO" });
}
