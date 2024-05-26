use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        p: [usize; n],
        c: [i64; n],
    }
    let mut res = -1 << 60;
    for i in 0..n {
        let mut v = i;
        let mut c_score = 0;
        let mut c_count = 0;
        loop {
            c_score += c[v];
            c_count += 1;
            v = p[v] - 1;
            if v == i {
                break;
            }
        }
        let mut score = 0;
        let mut count = 0;
        loop {
            score += c[v];
            count += 1;
            if count > k {
                break;
            }
            res = res.max(score + c_score.max(0) * ((k - count) / c_count));
            v = p[v] - 1;
            if v == i {
                break;
            }
        }
    }
    println!("{}", res);
}
