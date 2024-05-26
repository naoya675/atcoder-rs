use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut res = 0;
    for i in 0..n {
        if (i + 1).to_string().len() % 2 == 1 {
            res += 1;
        }
    }
    println!("{}", res);
}
