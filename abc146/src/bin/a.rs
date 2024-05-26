use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let res = match s.as_str() {
        "SUN" => 7,
        "MON" => 6,
        "TUE" => 5,
        "WED" => 4,
        "THU" => 3,
        "FRI" => 2,
        "SAT" => 1,
        _ => unreachable!(),
    };
    println!("{}", res);
}
