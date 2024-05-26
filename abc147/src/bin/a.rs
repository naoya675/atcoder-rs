use proconio::input;

fn main() {
    input! {
        a: [i32; 3],
    }
    let sum = a.iter().sum::<i32>();
    println!("{}", if sum < 22 { "win" } else { "bust" });
}
