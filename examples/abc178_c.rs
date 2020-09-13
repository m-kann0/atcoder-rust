use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    if n == 1 {
        return "0".to_string();
    }

    let mut ans: usize = 0;
    let mc = mod_comb::ModComb::init(MOD, n + 5);
    let mut e = mod_pow(8, n - 2);
    let einv = modinv(8, MOD as isize) as usize;
    let mut t = 2 * 2;
    for k in 2..=n {
        ans =  (ans + ((mc.calc(n, k) * ((e * (t - 2)) % MOD)) % MOD)) % MOD;
        e *= einv;
        e %= MOD;
        t *= 2;
        t %= MOD;
    }
    ans.to_string()
}

const MOD: usize = 1_000_000_007;

fn mod_pow(a: usize, b: usize) -> usize {
    let mut result = 1;
    for _ in 0..b {
        result = result * a % MOD;
    }
    return result
}

fn modinv(mut a: isize, mut m: isize) -> isize {
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b != 0 {
        let t: isize = a / b;

        a -= t * b;

        let temp = a;
        a = b;
        b = temp;

        u -= t * v;
        let temp = u;
        u = v;
        v = temp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u;
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
            r"2",
            "2"
        ),
        (
            r"1",
            "0"
        ),
        (
            r"3",
            "54"
        ),
        (
            r"869121",
            "2511445"
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