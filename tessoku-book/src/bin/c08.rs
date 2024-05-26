use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(usize, usize); n],
    }
    let mut vec = vec![true; 10000];
    let mut res = vec![];
    for i in 0..10000 {
        for (s, t) in &st {
            let mut power = 1;
            let mut count = 0;
            for _ in 0..4 {
                if i / power % 10 == s / power % 10 {
                    count += 1;
                }
                power *= 10;
            }
            match t {
                1 => vec[i] &= count == 4,
                2 => vec[i] &= count == 3,
                3 => vec[i] &= count < 3,
                _ => unreachable!(),
            }
        }
        if vec[i] {
            res.push(i);
        }
    }
    if res.len() == 1 {
        println!("{:04}", res[0]);
    } else {
        println!("Can't Solve");
    }
}
