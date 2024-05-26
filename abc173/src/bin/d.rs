use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();
    let mut res = 0;
    for i in 1..n {
        res += a[i / 2];
    }
    println!("{}", res);
}
