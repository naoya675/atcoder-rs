use itertools::Itertools;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    let mut res = 0.;
    let mut count = 0.;
    for per in (0..n).permutations(n) {
        for i in 1..n {
            let (xi, yi) = xy[per[i - 1]];
            let (xj, yj) = xy[per[i]];
            res += ((xi - xj) * (xi - xj) + (yi - yj) * (yi - yj)).sqrt();
        }
        count += 1.;
    }
    println!("{}", res / count);
}
