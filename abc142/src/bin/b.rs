use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        h: [i32; n],
    }
    let res = h.iter().filter(|&&h| h >= k).count();
    println!("{}", res);
}
