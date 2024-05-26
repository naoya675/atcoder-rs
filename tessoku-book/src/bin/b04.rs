use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut res = 0;
    for i in 0..8 {
        res += (n % 10) * 1 << i;
        n /= 10;
    }
    println!("{}", res);
}
