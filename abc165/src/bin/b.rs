use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    let mut res = 0;
    let mut money = 100;
    while money < x {
        money = money + money / 100;
        res += 1;
    }
    println!("{}", res);
}
