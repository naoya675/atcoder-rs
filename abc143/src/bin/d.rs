use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [i32; n],
    }
    l.sort();
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            res += l.lower_bound(&(l[i] + l[j])) - j - 1;
        }
    }
    println!("{}", res);
}
