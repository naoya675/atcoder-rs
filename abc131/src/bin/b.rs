use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
    }
    let sum = (0..n).map(|i| l + i).sum::<i32>();
    let mut min = l;
    for i in 0..n {
        if min.abs() > (l + i).abs() {
            min = l + i;
        }
    }
    println!("{}", sum - min);
}
