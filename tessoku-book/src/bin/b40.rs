use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut vec = vec![0; 100];
    for ai in a {
        vec[ai % 100] += 1;
    }
    let mut res = 0;
    res += vec[0] * (vec[0] - 1) / 2;
    for i in 1..50 {
        res += vec[i] * vec[100 - i];
    }
    res += vec[50] * (vec[50] - 1) / 2;
    println!("{}", res);
}
