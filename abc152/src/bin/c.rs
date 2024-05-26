use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut res = 0;
    let mut min = n;
    for p in p {
        min = min.min(p);
        if p == min {
            res += 1;
        }
    }
    println!("{}", res);
}
