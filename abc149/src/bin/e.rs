use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let check = |mi| {
        let mut count = 0;
        let mut res = 0;
        let mut j = n;
        for i in 0..n {
            while j > 0 && a[i] + a[j - 1] >= mi {
                j -= 1;
            }
            count += n - j;
            res += a[i] * (n - j) + (acc[n] - acc[j]);
        }
        (count, res)
    };
    let mut ok = 1 << 60;
    let mut ng = 0;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        let (count, _) = check(mi);
        if count >= m {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    let (count, res) = check(ng);
    println!("{}", res - (count - m) * ng);
}
