use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    for i in -200..200 {
        for j in -200..200 {
            if i * i * i * i * i - j * j * j * j * j == x {
                println!("{} {}", i, j);
                return;
            }
        }
    }
}
