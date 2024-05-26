use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }
    let m = 1 << 20;
    let mut count = vec![0; m];
    for a in a {
        count[a] += 1;
    }
    let mut res = count.iter().enumerate().fold(0, |acc, (i, &a)| acc + i * a);
    for (b, c) in bc {
        if b > c {
            res -= b.abs_diff(c) * count[b];
        } else {
            res += b.abs_diff(c) * count[b];
        }
        count[c] += count[b];
        count[b] = 0;
        println!("{}", res);
    }
}
