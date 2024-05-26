use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }
    let min = *x.iter().min().unwrap();
    let max = *x.iter().max().unwrap();
    let mut res = 1 << 30;
    for i in min..max + 1 {
        let mut sum = 0;
        for &x in &x {
            sum += (x - i) * (x - i);
        }
        res = res.min(sum);
    }
    println!("{}", res);
}
