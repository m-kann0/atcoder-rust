use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: i64 = iterator.next().unwrap().parse().unwrap();
    let a: i64 = iterator.next().unwrap().parse().unwrap();
    let b: i64 = iterator.next().unwrap().parse().unwrap();

    let mut ans: ModInt = ModInt::new(2).pow(n);
    ans -= ModInt::new(1);
    ans -= ModInt::comb(ModInt::new(n), ModInt::new(a));
    ans -= ModInt::comb(ModInt::new(n), ModInt::new(b));

    return format!("{}", ans);
}

use std::ops::{Add, AddAssign, SubAssign, Sub, MulAssign, Mul};
use std::fmt::{Display, Formatter, Error};

const MOD: i64 = 1_000_000_007;

#[derive(Copy, Clone)]
struct ModInt {
    value: i64
}

impl ModInt {
    fn new(v: i64) -> ModInt {
        ModInt { value: v }
    }

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

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 1 3",
            "7"
        ),
        (
            r"1000000000 141421 173205",
            "34076506"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
            println!("OK");
        } else {
            println!("NG");
            println!("    Expected: {}", expected);
            println!("    Actual  : {}", actual);

            all_ok = false;
        }
    }

    assert_eq!(all_ok, true);
}
