use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
        mut f: [i64; n],
    }
    a.sort();
    f.sort();
    f.reverse();
    let mut ok = 1 << 60;
    let mut ng = -1;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        let mut count = 0;
        for i in 0..n {
            if a[i] * f[i] > mi {
                count += a[i] - mi / f[i];
            }
        }
        if count <= k {
            ok = mi;
        } else {
            ng = mi;
        }
    }
    println!("{}", ok);
}
