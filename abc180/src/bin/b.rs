use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }
    let man = x.iter().map(|x| x.abs()).sum::<i64>();
    let euc = x.iter().map(|x| x.abs() * x.abs()).sum::<i64>() as f64;
    let che = x.iter().map(|x| x.abs()).max().unwrap();
    println!("{}", man);
    println!("{}", euc.sqrt());
    println!("{}", che);
}
