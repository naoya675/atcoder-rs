use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    let mut pow = 1;
    while n / pow > 0 {
        for i in 0..10 {
            if i < n / pow % 10 {
                res += i * (n / (pow * 10) * pow + pow);
            } else if i == n / pow % 10 {
                res += i * (n / (pow * 10) * pow + n % pow + 1);
            } else {
                res += i * (n / (pow * 10) * pow);
            }
        }
        pow *= 10;
    }
    println!("{}", res);
}
