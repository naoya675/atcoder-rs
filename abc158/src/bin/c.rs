use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    let mut res = -1;
    for i in 0..10000 {
        if (i * 8) / 100 == a && (i * 10) / 100 == b {
            res = i;
            break;
        }
    }
    println!("{}", res);
}
