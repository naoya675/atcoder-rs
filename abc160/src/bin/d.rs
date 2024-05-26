use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        mut y: usize,
    }
    x -= 1;
    y -= 1;
    let mut res = vec![0; n];
    for i in 0..n {
        for j in i..n {
            let dis = (j - i)
                .min(x.abs_diff(i) + y.abs_diff(j) + 1)
                .min(x.abs_diff(j) + y.abs_diff(i) + 1);
            res[dis] += 1;
        }
    }
    for i in 1..n {
        println!("{}", res[i]);
    }
}
