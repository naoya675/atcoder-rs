use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [i32; n],
        q: usize,
        x: [i32; q],
    }
    c.sort();
    let mut vec = vec![0; n + 1];
    for i in 0..n {
        vec[i + 1] = vec[i] + c[i];
    }
    for i in 0..q {
        let res = vec.upper_bound(&x[i]) - 1;
        println!("{}", res);
    }
}
