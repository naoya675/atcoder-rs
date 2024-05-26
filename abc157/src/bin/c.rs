use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, char); m],
    }
    let mut res = -1;
    for i in 0..1000 {
        let str = i.to_string().chars().collect::<Vec<_>>();
        if str.len() == n {
            if sc.iter().all(|&(s, c)| str[s - 1] == c) {
                res = i;
                break;
            }
        }
    }
    println!("{}", res);
}
