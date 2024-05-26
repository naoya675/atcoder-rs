use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut reverse = false;
    let mut a = (1..=n).collect::<Vec<usize>>();
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                if reverse {
                    a[n - x] = y;
                } else {
                    a[x - 1] = y;
                }
            }
            2 => {
                reverse = !reverse;
            }
            3 => {
                input! {
                    x: usize,
                }
                println!("{}", if reverse { a[n - x] } else { a[x - 1] });
            }
            _ => unreachable!(),
        }
    }
}
