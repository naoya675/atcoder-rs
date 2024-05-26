use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut ng = 0;
    let mut ok = 1_000_000_000;
    while ok - ng > 1 {
        let mi = (ng + ok) / 2;
        let mut sum = 0;
        for i in 0..n {
            sum += mi / a[i];
        }
        if sum < k {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ok);
}
