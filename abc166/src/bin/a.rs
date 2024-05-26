use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s.as_str() == "ABC" {
        println!("ARC");
    }
    if s.as_str() == "ARC" {
        println!("ABC");
    }
}
