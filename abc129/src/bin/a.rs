use proconio::input;

fn main() {
    input! {
        p: i32,
        q: i32,
        r: i32,
    }
    println!("{}", (p + q).min(q + r).min(r + p));
}
