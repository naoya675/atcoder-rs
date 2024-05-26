use proconio::input;

fn main() {
    input! {
        c: char,
    }
    println!("{}", ('a' as u8 + (1 + c as u8 - 'a' as u8) % 26) as char);
}
