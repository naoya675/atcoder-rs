use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
        a: [i32; n],
    }
    let sum = a.iter().sum::<i32>();
    let mut count = 0;
    for a in a {
        if 4 * m * a >= sum {
            count += 1;
        }
    }
    println!("{}", if count >= m { "Yes" } else { "No" });
}
