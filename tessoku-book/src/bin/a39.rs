use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(i32, i32); n],
    }
    lr.sort_by_key(|f| f.1);
    let mut res = 0;
    let mut last = 0;
    for (l, r) in lr {
        if last <= l {
            res += 1;
            last = r;
        }
    }
    println!("{}", res);
}
