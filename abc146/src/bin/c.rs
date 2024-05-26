use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    }
    let mut ok = 1_000_000_001;
    let mut ng = 0;
    let d = |n: i64| n.to_string().len() as i64;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        if a * mi + b * d(mi) <= x {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ng);
}
