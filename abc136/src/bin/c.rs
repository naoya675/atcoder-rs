use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [i32; n],
    }
    let mut res = true;
    for i in 1..n {
        if h[i - 1] > h[i] {
            h[i] += 1;
        }
        if h[i - 1] > h[i] {
            res = false;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
