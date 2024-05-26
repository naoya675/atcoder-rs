use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        ab: [(i32, char); n],
    }
    let mut res = 0;
    for (a, b) in ab {
        match b {
            'E' => res = res.max(l - a),
            'W' => res = res.max(a),
            _ => unreachable!(),
        }
    }
    println!("{}", res);
}
