use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut max = 0;
    let mut res = 0;
    for a in a {
        max = max.max(a);
        res += max - a;
    }
    println!("{}", res);
}
