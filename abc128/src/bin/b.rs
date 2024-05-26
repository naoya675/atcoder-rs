use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, i32); n],
    }
    let mut spi = sp
        .iter()
        .enumerate()
        .map(|(i, (s, p))| (s, -p, i))
        .collect::<Vec<_>>();
    spi.sort();
    for (_, _, i) in spi {
        println!("{}", i + 1);
    }
}
