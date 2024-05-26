use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [(usize, String); m],
    }
    let mut res_ac = 0;
    let mut res_wa = 0;
    let mut ac = vec![0; n];
    let mut wa = vec![0; n];
    for (p, s) in ps {
        match s.as_str() {
            "AC" => {
                ac[p - 1] += 1;
                if ac[p - 1] == 1 {
                    res_ac += ac[p - 1];
                    res_wa += wa[p - 1];
                }
            }
            "WA" => wa[p - 1] += 1,
            _ => unreachable!(),
        }
    }
    println!("{} {}", res_ac, res_wa);
}
