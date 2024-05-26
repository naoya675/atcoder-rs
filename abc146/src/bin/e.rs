use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // let acc = a
    //     .iter()
    //     .scan(0, |acc, a| {
    //         *acc += (*acc + a) % k;
    //         Some(*acc)
    //     })
    //     .collect::<Vec<_>>();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = (acc[i] + a[i]) % k;
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut res = 0;
    for i in 0..n + 1 {
        if i >= k {
            let key = (k + acc[i - k] - (i - k) % k) % k;
            *map.entry(key).or_insert(0) -= 1;
        }
        let key = (k + acc[i] - i % k) % k;
        res += *map.get(&key).unwrap_or(&0);
        *map.entry(key).or_insert(0) += 1;
    }
    println!("{}", res);
}
