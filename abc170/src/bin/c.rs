use proconio::input;

fn main() {
    input! {
        x: i32,
        n: usize,
        p: [i32; n],
    }
    let mut res = 0;
    for i in 0..200 {
        if !p.contains(&i) && (i - x).abs() < (res - x).abs() {
            res = i;
        }
    }
    println!("{}", res);
}
