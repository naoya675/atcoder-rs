use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut deque1 = VecDeque::new();
    let mut deque2 = VecDeque::new();
    for _ in 0..q {
        input! {
            query: char,
        }
        match query {
            'A' => {
                input! { x: String, }
                deque2.push_back(x);
            }
            'B' => {
                input! { x: String, }
                deque2.push_front(x);
            }
            'C' => {
                deque1.pop_front();
            }
            'D' => {
                println!("{}", deque1.front().unwrap());
            }
            _ => unreachable!(),
        }
        if deque1.len() < deque2.len() {
            deque1.push_back(deque2.pop_front().unwrap());
        }
    }
}
