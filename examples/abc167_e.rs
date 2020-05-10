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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mc: ModComb = ModComb::init(n);

    let mut ans: usize = 0;
    let mut col: usize = m;
    for x in (0..n).rev() {
        let mut now = col;
        now = now * mc.calc(n - 1, x) % MOD;
        if x <= k {
            ans = (ans + now) % MOD;
        }
        col = col * (m - 1) % MOD;
    }

    return ans.to_string();
}

const MOD: usize = 998244353;

struct ModComb {
    fac: Vec<usize>,
    finv: Vec<usize>,
    inv: Vec<usize>,
}

impl ModComb {
    fn init(max: usize) -> ModComb {
        let mut fac = vec![1; max];
        let mut finv = vec![1; max];
        let mut inv = vec![1; max];
        for i in 2..max {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }
        return ModComb {
            fac: fac,
            finv: finv,
            inv: inv,
        }
    }

    fn calc(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        return self.fac[n] * (self.finv[k] * self.finv[n - k] % MOD) % MOD;
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2 1",
            "6"
        ),
        (
            r"100 100 0",
            "73074801"
        ),
        (
            r"60522 114575 7559",
            "479519525"
        ),
        (
            r"60522 114575 7559",
            "479519525"
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