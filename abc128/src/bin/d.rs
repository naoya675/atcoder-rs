use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        v: [i32; n],
    }
    let mut res = 0;
    for l in 0..n + 1 {
        for r in 0..n + 1 {
            if n < l + r || k < l + r {
                continue;
            }
            let mut stone = vec![];
            for i in 0..l {
                stone.push(v[i]);
            }
            for i in n - r..n {
                stone.push(v[i]);
            }
            stone.sort();
            for i in 0..k - l - r {
                if i < stone.len() {
                    stone[i] = stone[i].max(0);
                }
            }
            res = res.max(stone.iter().sum::<i32>());
        }
    }
    println!("{}", res);
}
