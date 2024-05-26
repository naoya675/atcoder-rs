use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s == "Sunny".to_string() {
        println!("Cloudy");
    }
    if s == "Cloudy".to_string() {
        println!("Rainy");
    }
    if s == "Rainy".to_string() {
        println!("Sunny");
    }
}
