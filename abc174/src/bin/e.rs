use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut ok = 1 << 30;
    let mut ng = 0;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        let count = a.iter().map(|&a| (a + mi - 1) / mi - 1).sum::<i64>();
        if count <= k {
            ok = mi;
        } else {
            ng = mi;
        }
    }
    println!("{}", ok);
}
