use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut a = ab.iter().map(|(a, _)| a).collect::<Vec<_>>();
    let mut b = ab.iter().map(|(_, b)| b).collect::<Vec<_>>();
    a.sort();
    b.sort();
    if n % 2 == 0 {
        let res = (b[n / 2 - 1] + b[n / 2]) - (a[n / 2 - 1] + a[n / 2]) + 1;
        println!("{}", res);
    } else {
        let res = b[n / 2] - a[n / 2] + 1;
        println!("{}", res);
    }
}
