use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
    }
    a.sort();
    b.sort();
    b.reverse();
    let mut res = 0;
    for i in 0..n {
        res += a[i] * b[i];
    }
    println!("{}", res);
}
