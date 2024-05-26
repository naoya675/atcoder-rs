use proconio::input;

type Mint = ModInt<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
    let mut lower = vec![];
    let mut upper = vec![];
    for &a in &a {
        if a < 0 {
            lower.push(a);
        } else {
            upper.push(a);
        }
    }
    lower.sort();
    upper.sort();
    upper.reverse();
    if n == k {
        let lo = lower.iter().fold(Mint::new(1), |acc, &a| {
            acc * Mint::new((a + 1000000007) as u64)
        });
        let up = upper.iter().fold(Mint::new(1), |acc, &a| {
            acc * Mint::new((a + 1000000007) as u64)
        });
        println!("{}", lo * up);
        return;
    }
    if lower.len() == n {
        if k % 2 == 1 {
            lower.reverse();
        }
        let lo = lower[..k].iter().fold(Mint::new(1), |acc, &a| {
            acc * Mint::new((a + 1000000007) as u64)
        });
        println!("{}", lo);
        return;
    }
    let mut i = 0;
    let mut j = 0;
    let mut res = Mint::new(1);
    if k % 2 == 1 {
        j += 1;
        res *= Mint::new((upper[i] + 1000000007) as u64);
    }
    while i + j < k {
        if i + 1 < lower.len() && j + 1 < upper.len() {
            let lmax = lower[i] * lower[i + 1];
            let umax = upper[j] * upper[j + 1];
            if lmax > umax {
                res *= Mint::new(lmax as u64);
                i += 2;
            } else {
                res *= Mint::new(umax as u64);
                j += 2;
            }
        } else if i + 1 < lower.len() {
            let lmax = lower[i] * lower[i + 1];
            res *= Mint::new(lmax as u64);
            i += 2;
        } else if j + 1 < upper.len() {
            let umax = upper[j] * upper[j + 1];
            res *= Mint::new(umax as u64);
            j += 2;
        }
    }
    println!("{}", res);
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
