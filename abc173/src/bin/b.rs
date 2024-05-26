use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!("AC x {}", s.iter().filter(|s| s.as_str() == "AC").count());
    println!("WA x {}", s.iter().filter(|s| s.as_str() == "WA").count());
    println!("TLE x {}", s.iter().filter(|s| s.as_str() == "TLE").count());
    println!("RE x {}", s.iter().filter(|s| s.as_str() == "RE").count());
}
