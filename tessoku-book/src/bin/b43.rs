use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut res = vec![m; n];
    for ai in a {
        res[ai - 1] -= 1;
    }
    for ri in res {
        println!("{}", ri);
    }
}
