use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
    }
    let mut deque = VecDeque::from(s);
    let mut reverse = false;
    for _ in 0..q {
        input! { query: usize, }
        match query {
            1 => reverse = !reverse,
            2 => {
                input! { mut f: usize, c: char }
                if reverse {
                    f = f ^ 3;
                }
                match f {
                    1 => deque.push_front(c),
                    2 => deque.push_back(c),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    if reverse {
        println!("{}", deque.iter().rev().collect::<String>());
    } else {
        println!("{}", deque.iter().collect::<String>());
    }
}
