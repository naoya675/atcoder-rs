use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        b: i64,
        a: [i64; n],
        c: [i64; m],
    }
    let mut res = n * m * b;
    res += m * a.iter().sum::<i64>();
    res += n * c.iter().sum::<i64>();
    println!("{}", res);
}
