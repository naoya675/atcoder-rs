use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut height = vec![1; n];
    for i in 1..n {
        if s[i - 1] == 'A' {
            height[i] = height[i].max(height[i - 1] + 1);
        }
    }
    for i in (1..n).rev() {
        if s[i - 1] == 'B' {
            height[i - 1] = height[i - 1].max(height[i] + 1);
        }
    }
    println!("{}", height.iter().sum::<usize>());
}
