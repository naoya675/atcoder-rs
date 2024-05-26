use proconio::input;

fn main() {
    input! {
        l: i64,
        r: i64,
    }
    let mut res = 2019;
    'outer: for i in l..r + 1 {
        for j in i + 1..r + 1 {
            res = res.min((i * j) % 2019);
            if res == 0 {
                break 'outer;
            }
        }
    }
    println!("{}", res);
}
