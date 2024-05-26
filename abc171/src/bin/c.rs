use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }
    let mut res = vec![];
    while n > 0 {
        n -= 1;
        res.push(('a' as u8 + (n % 26) as u8) as char);
        n /= 26;
    }
    println!("{}", res.iter().rev().collect::<String>());
}
