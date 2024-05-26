use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i32; n],
    }
    let mut l = w.iter().sum::<i32>();
    let mut r = 0;
    let mut res = l;
    for w in w {
        l -= w;
        r += w;
        res = res.min((r - l).abs());
    }
    println!("{}", res);
}
