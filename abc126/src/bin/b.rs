use proconio::input;

fn main() {
    input! {
        s: i32,
    }
    let yymm = 1 <= (s % 100) && (s % 100) <= 12;
    let mmyy = 1 <= (s / 100) && (s / 100) <= 12;
    if yymm && mmyy {
        println!("AMBIGUOUS");
    } else if yymm {
        println!("YYMM");
    } else if mmyy {
        println!("MMYY");
    } else {
        println!("NA");
    }
}
