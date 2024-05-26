use proconio::input;

fn main() {
    input! {
        mut n: i32,
        k: i32,
    }
    let mut res = 0;
    while n > 0 {
        n /= k;
        res += 1;
    }
    println!("{}", res);
}
