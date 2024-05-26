use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        mut bc: [(i64, i64); m],
    }
    bc.extend(a.iter().map(|&a| (1, a)));
    bc.sort_by_key(|&(_, c)| -c);
    let mut i = n as i64;
    let mut res = 0;
    for (b, c) in bc {
        res += i.min(b) * c;
        i -= i.min(b);
    }
    println!("{}", res);
}
