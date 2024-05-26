use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 1..=n {
        // for j in (i..=n).step_by(i) {
        //     res += j;
        // }
        res += i * (n / i) * (n / i + 1) / 2;
    }
    println!("{}", res);
}
