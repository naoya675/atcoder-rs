use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let mut dist = vec![vec![-1; w]; h];
                let mut queue = VecDeque::new();
                dist[i][j] = 0;
                queue.push_back((i, j));
                while let Some((x, y)) = queue.pop_front() {
                    res = res.max(dist[x][y]);
                    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                        if x as isize + dx < 0 || y as isize + dy < 0 {
                            continue;
                        }
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;
                        if nx >= h || ny >= w {
                            continue;
                        }
                        if dist[nx][ny] == -1 && s[nx][ny] == '.' {
                            dist[nx][ny] = dist[x][y] + 1;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }
    println!("{}", res);
}
