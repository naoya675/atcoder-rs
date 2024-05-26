use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut vec = vec![0; t + 1];
    for (l, r) in lr {
        vec[l] += 1;
        vec[r] -= 1;
    }
    for i in 0..t {
        println!("{}", vec[i]);
        vec[i + 1] += vec[i];
    }
}
