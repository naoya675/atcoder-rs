use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    let res = a.windows(2).all(|a| a[0] != a[1]);
    println!("{}", if res { "YES" } else { "NO" });
}
