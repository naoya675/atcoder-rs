use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: Chars,
    }
    let mut queue = VecDeque::new();
    a[x - 1] = '@';
    queue.push_back(x - 1);
    while let Some(pos) = queue.pop_front() {
        if pos > 0 && a[pos - 1] == '.' {
            a[pos - 1] = '@';
            queue.push_back(pos - 1);
        }
        if pos < n - 1 && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            queue.push_back(pos + 1);
        }
    }
    println!("{}", a.iter().collect::<String>());
}
