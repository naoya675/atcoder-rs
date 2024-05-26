use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let mut res = 0;
    for i in 1..=n {
        for j in 1..=n {
            if 1 <= k - i - j && k - i - j <= n {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
