use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        l: [i32; n],
    }
    let mut d = 0;
    let mut res = 1;
    for l in l {
        d += l;
        if d <= x {
            res += 1;
        }
    }
    println!("{}", res);
}
