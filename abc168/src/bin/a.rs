use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let res = match n % 10 {
        2 | 4 | 5 | 7 | 9 => "hon",
        0 | 1 | 6 | 8 => "pon",
        3 => "bon",
        _ => unreachable!(),
    };
    println!("{}", res);
}
