use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut m = (n - 1) * (n - 2) / 2;
    if m < k {
        println!("-1");
        return;
    }
    let mut res = (1..n).map(|i| (1, i + 1)).collect::<Vec<_>>();
    for i in 1..n {
        for j in i + 1..n {
            if k < m {
                res.push((i + 1, j + 1));
                m -= 1;
            }
        }
    }
    println!("{}", res.len());
    for (i, j) in res {
        println!("{} {}", i, j);
    }
}
