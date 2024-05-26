use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: Chars,
    }
    let modulus: i64 = 1_000_000_007;
    let mut dp0 = vec![0; l.len() + 1];
    let mut dp1 = vec![0; l.len() + 1];
    dp0[0] = 1;
    for i in 0..l.len() {
        match l[i] {
            '0' => {
                // select 0
                dp1[i + 1] = (dp1[i + 1] + dp1[i]) % modulus;
                dp0[i + 1] = (dp0[i + 1] + dp0[i]) % modulus;
                // select 1
                dp1[i + 1] = (dp1[i + 1] + dp1[i] * 2) % modulus;
            }
            '1' => {
                // select 0
                dp1[i + 1] = (dp1[i + 1] + dp1[i]) % modulus;
                dp1[i + 1] = (dp1[i + 1] + dp0[i]) % modulus;
                // select 1
                dp1[i + 1] = (dp1[i + 1] + dp1[i] * 2) % modulus;
                dp0[i + 1] = (dp0[i + 1] + dp0[i] * 2) % modulus;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", (dp0[l.len()] + dp1[l.len()]) % modulus);
}
