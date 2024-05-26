use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        s: Chars,
    }
    let mut pre = vec![];
    let mut suf = vec![];
    let mut count = c;
    for i in 0..n {
        if s[i] == 'o' && count >= c {
            pre.push(i);
            count = 0;
        } else {
            count += 1;
        }
    }
    let mut count = c;
    for i in (0..n).rev() {
        if s[i] == 'o' && count >= c {
            suf.push(i);
            count = 0;
        } else {
            count += 1;
        }
    }
    suf.reverse();
    if pre.len() == k {
        for i in 0..k {
            if pre[i] == suf[i] {
                println!("{}", pre[i] + 1);
            }
        }
    }
}
