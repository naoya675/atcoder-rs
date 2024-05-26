use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut stack = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        match c {
            '(' => {
                stack.push_back(i);
            }
            ')' => {
                println!("{} {}", stack.pop_back().unwrap() + 1, i + 1);
            }
            _ => unreachable!(),
        }
    }
}
