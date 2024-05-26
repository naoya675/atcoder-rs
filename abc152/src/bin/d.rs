use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut count = vec![vec![0_i64; 10]; 10];
    for i in 1..=n {
        let mut head = i;
        while head / 10 > 0 {
            head /= 10;
        }
        let tail = i % 10;
        count[head][tail] += 1;
    }
    let mut res = 0;
    for i in 1..10 {
        for j in 1..10 {
            res += count[i][j] * count[j][i];
        }
    }
    println!("{}", res);
}
