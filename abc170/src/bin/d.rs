use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let max = 2_000_000;
    let mut vec = vec![0; max];
    for a in a {
        vec[a] += 1
    }
    let mut res = 0;
    for i in 0..max {
        if vec[i] == 0 {
            continue;
        }
        if vec[i] == 1 {
            res += 1;
        }
        for j in (i..max).step_by(i) {
            vec[j] = 0;
        }
    }
    println!("{}", res);
}
