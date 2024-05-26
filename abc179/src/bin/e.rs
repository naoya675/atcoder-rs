use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        m: usize,
    }
    let mut vec = vec![false; m];
    let mut x = x;
    let mut score = 0;
    let mut count = 0;
    loop {
        if vec[x] {
            let mut cycle = vec![];
            let mut sx = x;
            loop {
                cycle.push(sx);
                sx = (sx * sx) % m;
                if sx == x {
                    break;
                }
            }
            let c_count = (n - count) / cycle.len();
            let r_count = (n - count) % cycle.len();
            score += cycle.iter().sum::<usize>() * c_count;
            score += cycle[..r_count].iter().sum::<usize>();
            break;
        }
        vec[x] = true;
        score += x;
        count += 1;
        x = (x * x) % m;
        if count == n {
            break;
        }
    }
    println!("{}", score);

    // let mut dp = vec![vec![0; 100]; m];
    // let mut dp_acc = vec![vec![0; 100]; m];
    // for i in 0..m {
    //     dp[i][0] = (i * i) % m;
    //     dp_acc[i][0] = i;
    // }
    // for i in 0..34 {
    //     for j in 0..m {
    //         dp[j][i + 1] = dp[dp[j][i]][i];
    //         dp_acc[j][i + 1] = dp_acc[j][i] + dp_acc[dp[j][i]][i];
    //     }
    // }
    // let mut res = 0;
    // let mut x = x;
    // for i in 0..34 {
    //     if n & (1 << i) != 0 {
    //         res += dp_acc[x][i];
    //         x = dp[x][i];
    //     }
    // }
    // println!("{}", res);
}
