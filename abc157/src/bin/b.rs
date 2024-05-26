use proconio::input;

fn main() {
    input! {
        mut a: [i32; 9],
        n: usize,
        b: [i32; n],
    }
    for b in b {
        if let Some(i) = a.iter().position(|&a| a == b) {
            a[i] = 0;
        }
    }
    let res = a[0] + a[1] + a[2] == 0
        || a[3] + a[4] + a[5] == 0
        || a[6] + a[7] + a[8] == 0
        || a[0] + a[3] + a[6] == 0
        || a[1] + a[4] + a[7] == 0
        || a[2] + a[5] + a[8] == 0
        || a[0] + a[4] + a[8] == 0
        || a[2] + a[4] + a[6] == 0;
    println!("{}", if res { "Yes" } else { "No" });
}
