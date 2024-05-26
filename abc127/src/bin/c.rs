use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }
    let mut lmax = 0;
    let mut rmin = n;
    for (l, r) in lr {
        lmax = lmax.max(l);
        rmin = rmin.min(r);
    }
    println!("{}", if rmin >= lmax { rmin - lmax + 1 } else { 0 });
}
