use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }
    let mut ng = 0;
    let mut ok = n;
    while ok - ng > 1 {
        let mi = (ng + ok) / 2;
        if a[mi] < x {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ok + 1);
}
