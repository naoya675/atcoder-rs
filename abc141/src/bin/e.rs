use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut res = 0;
    for i in 0..n {
        let z = z_algorithm(&s[i..].to_vec());
        for j in 0..n - i {
            if z[j] < j + 1 {
                res = res.max(z[j]);
            }
        }
    }
    println!("{}", res);
}

pub fn z_algorithm(s: &Vec<char>) -> Vec<usize> {
    let mut res = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i + j < s.len() && s[i + j] == s[j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < s.len() && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
