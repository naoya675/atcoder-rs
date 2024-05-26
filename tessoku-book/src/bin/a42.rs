use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        ab: [(i32, i32); n],
    }
    let mut res = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            let mut cnt = 0;
            for &(a, b) in &ab {
                if i <= a && a <= i + k && j <= b && b <= j + k {
                    cnt += 1;
                }
            }
            res = res.max(cnt);
        }
    }
    println!("{}", res);
}
