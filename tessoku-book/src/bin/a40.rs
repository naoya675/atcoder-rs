use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt = vec![0_i64; 200];
    for ai in a {
        cnt[ai] += 1;
    }
    let mut res = 0;
    for i in 0..200 {
        res += cnt[i] * (cnt[i] - 1) * (cnt[i] - 2) / 6;
    }
    println!("{}", res);
}
