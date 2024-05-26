use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }
    let mut grundy = vec![0; 1_000_000];
    for i in 0..1_000_000 {
        let mut trans = [false, false, false];
        if i >= x {
            trans[grundy[i - x]] = true;
        }
        if i >= y {
            trans[grundy[i - y]] = true;
        }
        if trans[0] == false {
            grundy[i] = 0;
        } else if trans[1] == false {
            grundy[i] = 1;
        } else if trans[2] == false {
            grundy[i] = 2;
        }
    }
    let mut res = 0;
    for ai in a {
        res ^= grundy[ai];
    }
    println!("{}", if res > 0 { "First" } else { "Second" });
}
