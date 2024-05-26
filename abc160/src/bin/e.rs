use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut p: [i64; a],
        mut q: [i64; b],
        mut r: [i64; c],
    }
    p.sort_by(|a, b| b.cmp(a));
    q.sort_by(|a, b| b.cmp(a));
    r.sort_by(|a, b| b.cmp(a));
    let mut apple = vec![];
    p.iter().take(x).for_each(|&p| apple.push(p));
    q.iter().take(y).for_each(|&q| apple.push(q));
    r.iter().for_each(|&r| apple.push(r));
    apple.sort_by(|a, b| b.cmp(a));
    let res = apple[..x + y].iter().sum::<i64>();
    println!("{}", res);
}
