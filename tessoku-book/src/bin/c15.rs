use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(usize, usize); n],
    }
    let mut lr = lr.into_iter().map(|(l, r)| (l, r + k)).collect::<Vec<_>>();
    let lr_res = lr.clone();
    let mut pre = vec![0; 200_000];
    let mut pre_time = 0;
    lr.sort_by_key(|f| f.1);
    for &(l, r) in lr.iter() {
        if pre_time <= l {
            pre_time = r;
            pre[pre_time] += 1;
        }
    }
    for i in 1..200_000 {
        pre[i] += pre[i - 1];
    }
    let mut suf = vec![0; 200_000];
    let mut suf_time = 200_000;
    lr.sort_by_key(|f| f.0);
    for &(l, r) in lr.iter().rev() {
        if suf_time >= r {
            suf_time = l;
            suf[suf_time] += 1;
        }
    }
    for i in (1..200_000).rev() {
        suf[i - 1] += suf[i];
    }
    for &(l, r) in &lr_res {
        println!("{}", pre[l] + suf[r] + 1);
    }
}
