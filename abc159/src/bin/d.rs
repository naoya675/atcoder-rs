use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let a = a.iter().map(|a| a - 1).collect::<Vec<_>>();
    let mut vec = vec![0; n];
    for &a in &a {
        vec[a] += 1;
    }
    let res = vec.iter().map(|v| v * (v - 1) / 2).sum::<i64>();
    for &a in &a {
        println!("{}", res - (vec[a] - 1));
    }
}
