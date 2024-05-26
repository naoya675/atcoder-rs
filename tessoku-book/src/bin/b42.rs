use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut res1 = 0;
    let mut res2 = 0;
    let mut res3 = 0;
    let mut res4 = 0;
    for (a, b) in ab {
        if a + b > 0 {
            res1 += a + b;
        }
        if a - b > 0 {
            res2 += a - b;
        }
        if a - b < 0 {
            res3 += -(a - b);
        }
        if a + b < 0 {
            res4 += -(a + b);
        }
    }
    println!("{}", res1.max(res2).max(res3).max(res4));
}
