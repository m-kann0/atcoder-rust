use crate::mod_int::ModInt;

fn main() {
    const MOD: usize = 10;

    let a = ModInt::new(3, MOD);
    let b = ModInt::new(8, MOD);
    let c = a + b;
    println!("{}", c.value());
    assert_eq!(c.value(), 1);

    let mut a = ModInt::new(3, MOD);
    let b = ModInt::new(8, MOD);
    a += b;
    println!("{}", a.value());
    assert_eq!(a.value(), 1);

    let a = ModInt::new(3, MOD);
    let b = ModInt::new(4, MOD);
    let c = b - a;
    println!("{}", c.value());
    let c = a - b;
    println!("{}", c.value());
}

mod mod_int {
    use std::ops::{AddAssign, Add, Sub};

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModInt {
        value: usize,
        m: usize,
    }

    impl ModInt {
        pub fn new(value: usize, m: usize) -> Self {
            Self {
                value: value % m,
                m,
            }
        }

        pub fn value(self) -> usize {
            self.value
        }
    }

    impl Add for ModInt {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Self {
                value: (self.value + rhs.value) % self.m,
                m: self.m,
            }
        }
    }

    impl AddAssign for ModInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl Sub for ModInt {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                value: if self.value >= rhs.value {
                    (self.value - rhs.value) % self.m
                } else {
                    self.m - (rhs.value - self.value) % self.m
                },
                m: self.m,
            }
        }
    }
}
