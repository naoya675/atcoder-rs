use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut res = 0.;
    for i in 1..=n {
        let j = ((k + i - 1) / i).next_power_of_two();
        // let mut j = 1;
        // while i * j < k {
        //     j *= 2;
        // }
        res += 1. / n as f64 / j as f64;
    }
    println!("{}", res);
}
