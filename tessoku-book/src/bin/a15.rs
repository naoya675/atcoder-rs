use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = a[i];
    }
    b.sort();
    b.dedup();
    for i in 0..n {
        if let Ok(res) = b.binary_search(&a[i]) {
            print!("{} ", res + 1);
        }
        // let res = b.binary_search(&a[i]).unwrap();
        // print!("{} ", res + 1);
    }
    println!();
}
