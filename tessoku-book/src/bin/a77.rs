use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.push(0);
    a.push(l);
    a.sort();
    let mut ok = 1_000_000_000;
    let mut ng = 0;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        let mut cnt = 0;
        let mut cur = 0;
        for i in 1..n + 2 {
            cur += a[i] - a[i - 1];
            if cur > mi {
                cnt += 1;
                cur = 0;
            }
        }
        if cnt > k {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ok);
}
