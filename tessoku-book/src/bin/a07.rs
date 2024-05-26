use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut vec = vec![0; d + 1];
    for (l, r) in lr {
        vec[l - 1] += 1;
        vec[r] -= 1;
    }
    for i in 0..d {
        println!("{}", vec[i]);
        vec[i + 1] += vec[i];
    }
}
