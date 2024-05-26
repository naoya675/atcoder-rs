use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut queue = BinaryHeap::new();
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: i32,
                }
                queue.push(-x);
            }
            2 => {
                println!("{}", -queue.peek().unwrap());
            }
            3 => {
                queue.pop();
            }
            _ => unreachable!(),
        }
    }
}
