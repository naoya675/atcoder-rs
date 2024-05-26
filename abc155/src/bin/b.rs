use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let res = a.iter().all(|a| a % 2 == 1 || (a % 6 == 0 || a % 10 == 0));
    println!("{}", if res { "APPROVED" } else { "DENIED" });
}
