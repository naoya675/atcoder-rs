use proconio::input;

fn main() {
    input! {
        k: i32,
    }
    let mut rem = 0;
    let mut res = -1;
    for i in 1..=k {
        rem = (rem * 10 + 7) % k;
        if rem == 0 {
            res = i;
            break;
        }
    }
    println!("{}", res);
}
