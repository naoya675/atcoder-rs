use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }
    let mut res = 0;
    for i in 1..n - 1 {
        if p[i - 1] < p[i] && p[i] < p[i + 1] {
            res += 1;
        }
        if p[i - 1] > p[i] && p[i] > p[i + 1] {
            res += 1;
        }
    }
    println!("{}", res);
}
