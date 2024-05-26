use proconio::input;

fn main() {
    input! {
        n: usize,
        _x: usize,
        _y: usize,
        a: [usize; n],
    }
    let mut res = 0;
    for ai in a {
        if ai % 5 == 0 || ai % 5 == 1 {
            res ^= 0;
        }
        if ai % 5 == 2 || ai % 5 == 3 {
            res ^= 1;
        }
        if ai % 5 == 4 {
            res ^= 2;
        }
    }
    println!("{}", if res > 0 { "First" } else { "Second" });
}
