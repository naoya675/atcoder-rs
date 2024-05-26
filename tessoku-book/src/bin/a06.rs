use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        lr: [(usize, usize); q],
    }
    let mut vec = vec![0; n + 1];
    for i in 0..n {
        vec[i + 1] = vec[i] + a[i];
    }
    for (l, r) in lr {
        println!("{}", vec[r] - vec[l - 1]);
    }
}
