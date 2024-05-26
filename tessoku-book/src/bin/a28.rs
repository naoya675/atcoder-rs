use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, i32); n],
    }
    let mut res = 0;
    for (t, a) in ta {
        match t {
            '+' => res += a,
            '-' => res -= a,
            '*' => res *= a,
            _ => unreachable!(),
        }
        res = (res + 10000) % 10000;
        println!("{}", res);
    }
}
