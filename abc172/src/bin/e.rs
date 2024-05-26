use proconio::input;

type Mint = ModInt<1000000007>;
type Mcom = ModCombinatorial<1000000007>;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mc = Mcom::new(m + 1);
    let mut res = Mint::new(0);
    for i in 0..=n {
        if i % 2 == 0 {
            res += mc.comb(n, i) * mc.perm(m, i) * mc.perm(m - i, n - i) * mc.perm(m - i, n - i);
        } else {
            res -= mc.comb(n, i) * mc.perm(m, i) * mc.perm(m - i, n - i) * mc.perm(m - i, n - i);
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

#[derive(Debug, Clone)]
pub struct ModCombinatorial<const MOD: u64> {
    fact: Vec<ModInt<MOD>>,
    finv: Vec<ModInt<MOD>>,
}

impl<const MOD: u64> ModCombinatorial<MOD> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![ModInt::<MOD>::new(1); n + 1];
        let mut finv = vec![ModInt::<MOD>::new(1); n + 1];
        for i in 0..n {
            fact[i + 1] = fact[i] * ModInt::<MOD>::new((i + 1) as u64);
        }
        finv[n] = fact[n].inv();
        for i in (0..n).rev() {
            finv[i] = finv[i + 1] * ModInt::<MOD>::new((i + 1) as u64);
        }
        Self { fact, finv }
    }

    pub fn fact(&self, n: usize) -> ModInt<MOD> {
        self.fact[n]
    }

    pub fn finv(&self, n: usize) -> ModInt<MOD> {
        self.finv[n]
    }

    // permutation
    pub fn perm(&self, n: usize, r: usize) -> ModInt<MOD> {
        // assert!(r <= n);
        if r > n {
            return ModInt::<MOD>::new(0);
        }
        self.fact[n] * self.finv[n - r]
    }

    // combination
    pub fn comb(&self, n: usize, r: usize) -> ModInt<MOD> {
        // assert!(r <= n);
        if r > n {
            return ModInt::<MOD>::new(0);
        }
        self.fact[n] * self.finv[r] * self.finv[n - r]
    }

    // homogeneous product
    pub fn homo(&self, n: usize, r: usize) -> ModInt<MOD> {
        self.comb(n + r - 1, r)
    }
}
