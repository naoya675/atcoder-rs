use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut count = vec![0; n];
    for _ in 0..k {
        input! {
            d: usize,
            a: [usize; d],
        }
        for a in a {
            count[a - 1] = 1;
        }
    }
    let res = count.iter().filter(|&&c| c == 0).count();
    println!("{}", res);
}
