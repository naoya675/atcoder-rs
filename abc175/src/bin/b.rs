use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [i32; n],
    }
    l.sort();
    let mut res = 0;
    for i in 0..n {
        for j in i..n {
            for k in j..n {
                if l[i] != l[j] && l[j] != l[k] && l[i] + l[j] > l[k] {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}
