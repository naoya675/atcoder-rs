use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }
    ab.sort_by_key(|&(_, b)| b);
    let mut time = 0;
    let mut res = true;
    for (a, b) in ab {
        time += a;
        if time > b {
            res = false;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
