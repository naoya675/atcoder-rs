use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    println!("{}", s.matches("ABC").count());
}
