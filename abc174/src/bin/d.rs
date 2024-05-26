use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        c: Chars,
    }
    let red = c.iter().filter(|&&c| c == 'R').count();
    println!("{}", c[..red].iter().filter(|&&c| c == 'W').count());
}
