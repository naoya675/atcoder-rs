use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        m: i32,
        a: [i32; n - 1],
    }
    let res = m * n as i32 - a.iter().sum::<i32>();
    println!("{}", if res <= k { res.max(0) } else { -1 });
}
