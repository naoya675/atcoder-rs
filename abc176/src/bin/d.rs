use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        (ch, cw): (usize, usize),
        (dh, dw): (usize, usize),
        s: [Chars; h],
    }
    let mut res = vec![vec![1 << 30; w]; h];
    let mut heap = BinaryHeap::new();
    res[ch - 1][cw - 1] = 0;
    heap.push((-res[ch - 1][cw - 1], (ch - 1, cw - 1)));
    while let Some((d, (x, y))) = heap.pop() {
        if -d > res[x][y] {
            continue;
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if x as isize + dx < 0 || y as isize + dy < 0 {
                continue;
            }
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx >= h || ny >= w {
                continue;
            }
            if s[nx][ny] == '.' && res[nx][ny] > res[x][y] {
                res[nx][ny] = res[x][y];
                heap.push((-res[nx][ny], (nx, ny)));
            }
        }
        for dx in -2..=2 {
            for dy in -2..=2 {
                if x as isize + dx < 0 || y as isize + dy < 0 {
                    continue;
                }
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx >= h || ny >= w {
                    continue;
                }
                if s[nx][ny] == '.' && res[nx][ny] > res[x][y] + 1 {
                    res[nx][ny] = res[x][y] + 1;
                    heap.push((-res[nx][ny], (nx, ny)));
                }
            }
        }
    }
    if res[dh - 1][dw - 1] == 1 << 30 {
        println!("{}", -1);
    } else {
        println!("{}", res[dh - 1][dw - 1]);
    }
}
