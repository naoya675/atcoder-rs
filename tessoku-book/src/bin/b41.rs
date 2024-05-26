use proconio::input;

fn main() {
    input! {
        mut x: i32,
        mut y: i32,
    }
    let mut res = vec![];
    while (x, y) != (1, 1) {
        res.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    println!("{}", res.len());
    for (x, y) in res.iter().rev() {
        println!("{} {}", x, y);
    }
}
