use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
        d: [i32; n],
    }
    let mut ab = vec![0; n * n];
    let mut cd = vec![0; n * n];
    for i in 0..n {
        for j in 0..n {
            ab[n * i + j] = a[i] + b[j];
        }
    }
    for i in 0..n {
        for j in 0..n {
            cd[n * i + j] = c[i] + d[j];
        }
    }
    ab.sort();
    cd.sort();
    let mut res = false;
    let mut j = n * n - 1;
    for i in 0..n * n {
        while j > 0 && ab[i] + cd[j] > k {
            j -= 1;
        }
        if ab[i] + cd[j] == k {
            res = true;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
