use proconio::input;

fn main() {
    input! {
        d: usize,
        x: i32,
        a: [i32; d - 1],
        q: usize,
        st: [(usize, usize); q],
    }
    let mut stock = vec![x; d];
    for i in 1..d {
        stock[i] = stock[i - 1] + a[i - 1];
    }
    for (s, t) in st {
        if stock[s - 1] == stock[t - 1] {
            println!("Same");
        } else {
            println!("{}", if stock[s - 1] > stock[t - 1] { s } else { t });
        }
    }
}
