use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        (sy, sx): (usize, usize),
        (gy, gx): (usize, usize),
        cr: [Chars; r],
    }
    let mut res = vec![vec![-1; c]; r];
    let mut queue = VecDeque::new();
    res[sy - 1][sx - 1] = 0;
    queue.push_back((sy - 1, sx - 1));
    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;
            if res[ny][nx] == -1 && cr[ny][nx] == '.' {
                res[ny][nx] = res[y][x] + 1;
                queue.push_back((ny, nx));
            }
        }
    }
    println!("{}", res[gy - 1][gx - 1]);
}
