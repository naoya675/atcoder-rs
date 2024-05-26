use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut res = 0;
    if n % 2 == 0 {
        let mut power = 10;
        while power <= n {
            res += n / power;
            power *= 5;
        }
    }
    println!("{}", res);
}
