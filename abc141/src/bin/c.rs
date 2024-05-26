use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [usize; q],
    }
    let mut res = vec![q; n];
    for a in a {
        res[a - 1] -= 1;
    }
    for i in 0..n {
        println!("{}", if k > res[i] { "Yes" } else { "No" });
    }
}
