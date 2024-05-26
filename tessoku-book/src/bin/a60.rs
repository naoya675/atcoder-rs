use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
    stack.push_back((-1, 1_000_000_001));
    for i in 0..n {
        while let Some((day, stock)) = stack.pop_back() {
            if stock > a[i] {
                print!("{} ", day);
                stack.push_back((day, stock));
                stack.push_back(((i + 1) as i32, a[i]));
                break;
            }
        }
    }
    println!();
}
