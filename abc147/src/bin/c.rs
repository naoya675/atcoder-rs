use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut honest = vec![vec![]; n];
    let mut unkind = vec![vec![]; n];
    for i in 0..n {
        input! {
            a: usize,
            xy: [(usize, usize); a],
        }
        for (x, y) in xy {
            match y {
                1 => honest[i].push(x - 1),
                0 => unkind[i].push(x - 1),
                _ => unreachable!(),
            }
        }
    }
    let mut res = 0;
    'outer: for bit in 0..1 << n {
        let mut check = vec![0; n];
        for i in 0..n {
            if bit & (1 << i) != 0 {
                check[i] = 1;
            }
        }
        for i in 0..n {
            if check[i] == 0 {
                continue;
            }
            for &j in &honest[i] {
                if check[j] != 1 {
                    continue 'outer;
                }
            }
            for &j in &unkind[i] {
                if check[j] != 0 {
                    continue 'outer;
                }
            }
        }
        res = res.max(check.iter().sum::<usize>());
    }
    println!("{}", res);
}
