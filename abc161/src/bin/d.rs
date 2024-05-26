use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        k: i32,
    }
    let mut queue: VecDeque<i64> = VecDeque::new();
    for i in 1..10 {
        queue.push_back(i);
    }
    for _ in 1..k {
        if let Some(q) = queue.pop_front() {
            if q % 10 != 0 {
                queue.push_back(q * 10 + (q % 10) - 1);
            }
            queue.push_back(q * 10 + (q % 10));
            if q % 10 != 9 {
                queue.push_back(q * 10 + (q % 10) + 1);
            }
        }
    }
    if let Some(q) = queue.pop_front() {
        println!("{}", q);
    }
}
