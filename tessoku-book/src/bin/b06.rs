use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut vec = vec![0; n + 1];
    for i in 0..n {
        if a[i] == 1 {
            vec[i + 1] = vec[i] + 1;
        } else {
            vec[i + 1] = vec[i] - 1;
        }
    }
    for (l, r) in lr {
        if vec[r] - vec[l - 1] > 0 {
            println!("win");
        } else if vec[r] - vec[l - 1] < 0 {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
