use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let res = match s.as_str() {
        "SSS" => 0,
        "RRR" => 3,
        "RRS" | "SRR" => 2,
        _ => 1,
    };
    println!("{}", res);
}
