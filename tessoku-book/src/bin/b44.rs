use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
        query: [(usize, usize, usize); q],
    }
    let mut row = (0..n).collect::<Vec<usize>>();
    for (query, x, y) in query {
        match query {
            1 => {
                row.swap(x - 1, y - 1);
            }
            2 => {
                println!("{}", a[row[x - 1]][y - 1]);
            }
            _ => unreachable!(),
        }
    }
}
