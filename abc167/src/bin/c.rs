use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: i32,
        ca: [[i32; m + 1]; n],
    }
    let mut res = 1 << 30;
    for bit in 0..1 << n {
        let mut money = 0;
        let mut skill = vec![0; m + 1];
        for i in 0..n {
            if bit & (1 << i) != 0 {
                money += ca[i][0];
                for j in 1..m + 1 {
                    skill[j] += ca[i][j];
                }
            }
        }
        if skill[1..].iter().all(|&a| a >= x) {
            res = res.min(money);
        }
    }
    println!("{}", if res == 1 << 30 { -1 } else { res });
}
