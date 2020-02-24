use std::ops::{Add, AddAssign, SubAssign, Sub, MulAssign, Mul};
use std::fmt::{Display, Formatter, Error};

const MOD: i64 = 1_000_000_007;

#[derive(Copy, Clone)]
struct ModInt {
    value: i64
}

impl ModInt {
    fn pow(self, x: i64) -> ModInt {
        if x == 0 {
            return ModInt { value: 1 };
        }

        return if x % 2 == 0 {
            let t = ModInt::pow(self, x / 2);
            t * t
        } else {
            self * ModInt::pow(self, x - 1)
        }
    }

    fn comb(n: ModInt, k: ModInt) -> ModInt {
        let mut numerator = ModInt { value: 1 };
        let mut denominator = ModInt { value: 1 };
        for i in 0..k.value {
            numerator = numerator * (n - ModInt { value: i });
            denominator = denominator * (k - ModInt { value: i });
        }
        numerator * denominator.pow(MOD - 2)
    }
}

impl Display for ModInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}

impl Add for ModInt {
    type Output = ModInt;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = ModInt { value: self.value };
        result += rhs;
        result
    }
}

impl Sub for ModInt {
    type Output = ModInt;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = ModInt { value: self.value };
        result -= rhs;
        result
    }
}

impl Mul for ModInt {
    type Output = ModInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = ModInt { value: self.value };
        result *= rhs;
        result
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if self.value > MOD {
            self.value -= MOD;
        }
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
        if self.value < 0 {
            self.value += MOD;
        }
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = self.value * rhs.value % MOD;
    }
}

fn main() {
    let a = ModInt { value: 10 };
    let b = ModInt { value: 5 };
    let c = a + b;
    println!("{}", c);

    let mut a = ModInt { value: 10 };
    let b = ModInt { value: 5 };
    a += b;
    println!("{}", a);

    println!("{}", ModInt { value: 2 }.pow(4));
    println!("{}", ModInt::comb(ModInt { value: 5 }, ModInt { value: 2 }));
}
