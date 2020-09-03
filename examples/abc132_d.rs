use std::io::Read;

const MOD: usize = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let blue = k;
    let red = n - k;

    let mut result = String::new();

    let mc = mod_comb::ModComb::init(MOD, 5000);
    for i in 1..=k {
        if red + 1 < i {
            result.push_str("0\n");
            continue;
        }
        let b = mc.calc(blue - 1, i - 1);
        let r = mc.calc(red + 1, red + 1 - i);
        result.push_str(&format!("{}\n", (b * r) % MOD));
    }

    result.trim().to_string()
}

mod mod_comb {
    pub struct ModComb {
        modulo: usize,
        fac: Vec<usize>,
        finv: Vec<usize>,
    }

    impl ModComb {
        pub fn init(modulo: usize, max: usize) -> ModComb {
            let mut fac = vec![1; max];
            let mut finv = vec![1; max];
            let mut inv = vec![1; max];
            for i in 2..max {
                fac[i] = fac[i - 1] * i % modulo;
                inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
                finv[i] = finv[i - 1] * inv[i] % modulo;
            }
            ModComb {
                modulo,
                fac,
                finv,
            }
        }

        pub fn calc(&self, n: usize, k: usize) -> usize {
            if n < k {
                return 0;
            }
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.modulo) % self.modulo
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3",
            "3
6
1"
        ),
        (
            r"2000 3",
            "1998
3990006
327341989"
        ),
        (
            r"2000 2000",
            "1"
        ),
        (
            r"3 3",
            "1"
        ),
        (
            r"1 1",
            "1"
        ),
        (
            r"2 1",
            "2"
        ),
        (
            r"3 1",
            "3"
        ),
        (
            r"100 100",
            "2
1"
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