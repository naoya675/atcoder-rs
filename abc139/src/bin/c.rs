use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut vec = vec![0; n];
    let mut res = 0;
    for i in 1..n {
        if h[i] <= h[i - 1] {
            vec[i] = vec[i - 1] + 1;
        }
        res = res.max(vec[i]);
    }
    println!("{}", res);
}
