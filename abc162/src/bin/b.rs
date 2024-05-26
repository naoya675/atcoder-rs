use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut res = 0;
    for i in 1..=n {
        if i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        res += i;
    }
    println!("{}", res);
}
