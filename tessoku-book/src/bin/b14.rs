use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let (al, ar) = a.split_at(n / 2);
    let mut als = vec![];
    let mut ars = vec![];
    for bit in 0..1 << al.len() {
        let mut sum = 0;
        for i in 0..al.len() {
            if bit & (1 << i) != 0 {
                sum += al[i];
            }
        }
        als.push(sum);
    }
    for bit in 0..1 << ar.len() {
        let mut sum = 0;
        for i in 0..ar.len() {
            if bit & (1 << i) != 0 {
                sum += ar[i];
            }
        }
        ars.push(sum);
    }
    als.sort();
    ars.sort();
    let mut res = false;
    for a in als {
        if ars.binary_search(&(k - a)).is_ok() {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
