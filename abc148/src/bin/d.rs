use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut len = 1;
    let mut res = 0;
    for a in a {
        if len == a {
            len += 1;
        } else {
            res += 1;
        }
    }
    println!("{}", if len == 1 { -1 } else { res });
}
