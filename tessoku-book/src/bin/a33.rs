use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut res = 0;
    for ai in a {
        res ^= ai;
    }
    println!("{}", if res > 0 { "First" } else { "Second" })
}
