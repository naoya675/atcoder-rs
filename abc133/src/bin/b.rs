use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i32; d]; n],
    }
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut dist = 0;
            for k in 0..d {
                dist += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }
            if is_square(dist) {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

fn is_square(n: i32) -> bool {
    for i in 0..=n {
        if i * i == n {
            return true;
        }
    }
    false
}
