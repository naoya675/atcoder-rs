use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }
    let mut z = vec![];
    let mut w = vec![];
    for (x, y) in xy {
        z.push(x + y);
        w.push(x - y);
    }
    z.sort();
    w.sort();
    println!("{}", (z[n - 1] - z[0]).max(w[n - 1] - w[0]));
}
