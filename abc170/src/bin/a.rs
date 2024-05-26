use proconio::input;

fn main() {
    input! {
        x: [i32; 5],
    }
    println!("{}", 15 - x.iter().sum::<i32>());
}
