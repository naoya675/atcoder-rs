use std::collections::{HashMap, HashSet};

use proconio::input;

type Mint = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let ab = ab
        .iter()
        .map(|&(a, b)| {
            if (a, b) == (0, 0) {
                return (0, 0);
            }
            if a == 0 {
                return (0, 1);
            }
            if b == 0 {
                return (1, 0);
            }
            let gcd = gcd(a.abs(), b.abs());
            let mut a = a / gcd;
            let mut b = b / gcd;
            if b < 0 {
                a = -a;
                b = -b;
            }
            (a, b)
        })
        .collect::<Vec<_>>();
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut count = 0;
    for (a, b) in ab {
        if (a, b) == (0, 0) {
            count += 1;
        }
        *map.entry((a, b)).or_default() += 1;
    }
    let mut select = HashSet::new();
    select.insert((0, 0));
    let mut res = Mint::new(1);
    for (&(a, b), &c) in &map {
        if select.contains(&(a, b)) {
            continue;
        }
        let (mut ai, mut bi) = (b, -a);
        if bi < 0 {
            ai = -ai;
            bi = -bi;
        }
        select.insert((a, b));
        select.insert((ai, bi));
        if let Some(&ci) = map.get(&(ai, bi)) {
            res *= Mint::new(2).power(c as u64) + Mint::new(2).power(ci as u64) - Mint::new(1);
        } else {
            res *= Mint::new(2).power(c as u64);
        }
    }
    println!("{}", res + Mint::new(count) - Mint::new(1));
}

fn gcd(a: i64, b: i64) -> i64 {
    assert!(a > 0);
    assert!(b > 0);
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModInt<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> ModInt<MOD> {
    pub fn new(n: u64) -> Self {
        Self {
            value: (n % MOD),
            // value: (n.rem_euclid(MOD)),
        }
    }

    pub fn value(&self) -> u64 {
        self.value % MOD
    }

    pub fn power(&self, mut n: u64) -> Self {
        let mut value = self.value;
        let mut res = 1;
        while n > 0 {
            if n & 1 != 0 {
                res = (res * value) % MOD;
            }
            value = (value * value) % MOD;
            n >>= 1;
        }
        Self { value: res }
    }

    pub fn pow(&self, n: Self) -> Self {
        self.power(n.value)
    }

    pub fn inv(&self) -> Self {
        self.power(MOD - 2)
    }
}

impl<const MOD: u64> std::ops::Add for ModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: (self.value + rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: u64> std::ops::Sub for ModInt<MOD> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        if self.value < rhs.value {
            self.value += MOD;
        }
        Self {
            value: (self.value - rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: u64> std::ops::Mul for ModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: (self.value * rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: u64> std::ops::Div for ModInt<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.value == 0 {
            panic!();
        }
        self * rhs.inv()
    }
}

impl<const MOD: u64> std::ops::DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: u64> std::fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
