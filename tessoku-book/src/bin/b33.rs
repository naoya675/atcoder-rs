use proconio::input;

fn main() {
    input! {
        n: usize,
        _h: usize,
        _w: usize,
        ab: [(usize, usize); n],
    }
    let mut res = 0;
    for (a, b) in ab {
        res ^= a - 1;
        res ^= b - 1;
    }
    println!("{}", if res > 0 { "First" } else { "Second" });
}
