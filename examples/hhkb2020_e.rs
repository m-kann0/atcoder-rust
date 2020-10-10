use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut nz = 0;
    let mut counts: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); w]; h];
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '#' {
                counts[i][j] = (0, 0);
                continue;
            }
            nz += 1;
            let vertical = if i > 0 && map[i - 1][j] != '#' {
                counts[i - 1][j].0
            } else {
                let mut vertical = 1;
                for k in (0..i).rev() {
                    if map[k][j] == '#' {
                        break;
                    }
                    vertical += 1;
                }
                for k in (i + 1)..h {
                    if map[k][j] == '#' {
                        break;
                    }
                    vertical += 1;
                }
                vertical
            };
            let horizontal = if j > 0 && map[i][j - 1] != '#' {
                counts[i][j - 1].1
            } else {
                let mut horizontal = 1;
                for k in (0..j).rev() {
                    if map[i][k] == '#' {
                        break;
                    }
                    horizontal += 1;
                }
                for k in (j + 1)..w {
                    if map[i][k] == '#' {
                        break;
                    }
                    horizontal += 1;
                }
                horizontal
            };
            counts[i][j] = (vertical, horizontal);
        }
    }

    let mut ans = mod_int::ModInt::new(0);
    let mut memo: HashMap<usize, mod_int::ModInt<usize>> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let count = counts[i][j].0 + counts[i][j].1;
            if count == 0 {
                continue;
            }
            let count = count - 1;
            if memo.contains_key(&count) {
                ans += *memo.get(&count).unwrap();
                continue;
            }
            let mut add = mod_int::ModInt::new(2).pow(nz - 1);
            let mut a = mod_int::ModInt::new(0);
            for _ in 0..count {
                a += add;
                add /= 2;
            }
            memo.insert(count, a);
            ans += a;
        }
    }
    ans.to_string()
}

pub mod mod_int {
    use std::fmt;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;

    const MOD: Num = 1_000_000_007;

    #[derive(Copy, Clone)]
    pub struct ModInt<T: Clone + Copy> {
        pub v: T,
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            let mut t = rhs + self.v;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt::new(t)
        }
    }

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.v
        }
    }

    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other
        }
    }

    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.v < rhs { self.v + MOD } else { self.v };
            ModInt::new(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.v
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other
        }
    }

    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            ModInt::new((self.v * rhs) % MOD)
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.v
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs
        }
    }

    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs
        }
    }

    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs % MOD;
            }
            self * ModInt::new(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.v
        }
    }

    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }

    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl fmt::Display for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.v)
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: Num) -> ModInt<Num> {
            let mut result = ModInt::new(1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }

        pub fn new(v: Num) -> ModInt<Num> {
            if v >= MOD {
                ModInt { v: v % MOD }
            } else {
                ModInt { v }
            }
        }
    }
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 5
..#..",
            "48"
        ),
        (
            r"2 3
..#
#..",
            "52"
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