use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut zero = 0;
    let mut lower = vec![];
    let mut upper = vec![];
    for a in a {
        if a == 0 {
            zero += 1;
        }
        if a < 0 {
            lower.push(a);
        }
        if a > 0 {
            upper.push(a);
        }
    }
    lower.reverse();
    let mut ok = 1 << 60;
    let mut ng = -1 << 60;
    while ok - ng > 1 {
        let mi = (ok + ng) / 2;
        let mut count = 0;
        if 0 <= mi {
            count += get_zero(n, zero);
        }
        count += get_lower(mi, &lower, &upper);
        count += get_upper(mi, &lower, &upper);
        if count < k {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ok);
}

fn get_zero(n: usize, zero: usize) -> usize {
    if zero == 0 {
        return 0;
    }
    let res = zero * (n - zero) + zero * (zero - 1) / 2;
    res
}

fn get_lower(p: i64, lower: &Vec<i64>, upper: &Vec<i64>) -> usize {
    let mut res = 0;
    let mut j = 0;
    for i in (0..lower.len()).rev() {
        while j < upper.len() && p < lower[i] * upper[j] {
            j += 1;
        }
        res += upper.len() - j;
    }
    res
}

fn get_upper(p: i64, lower: &Vec<i64>, upper: &Vec<i64>) -> usize {
    let mut res = 0;
    let mut j = lower.len() - 1;
    for i in 0..lower.len() {
        while i < j && p < lower[i] * lower[j] {
            j -= 1;
        }
        res += j - j.min(i);
    }
    let mut j = upper.len() - 1;
    for i in 0..upper.len() {
        while i < j && p < upper[i] * upper[j] {
            j -= 1;
        }
        res += j - j.min(i);
    }
    res
}
