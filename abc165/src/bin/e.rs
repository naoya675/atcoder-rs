use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut s = 1;
    let mut t = n;
    for i in (1..=m).rev() {
        if i % 2 == 1 {
            println!("{} {}", s, s + i);
            s += 1;
        } else {
            println!("{} {}", t - i, t);
            t -= 1;
        }
    }
}
