use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut x: usize = iterator.next().unwrap().parse().unwrap();
    let mut y: usize = iterator.next().unwrap().parse().unwrap();
    if (x + y) % 3 != 0 {
        return "0".to_string();
    }
    if x > y {
        let temp = x;
        x = y;
        y = temp;
    }

    if 2 * x < y {
        return "0".to_string();
    }

    let n: usize = (2 * y - x) / 3;
    let m: usize = (2 * x - y) / 3;

    let mc: ModComb = ModComb::init(n + m + 1);
    let ans = mc.calc(n + m, n);

    return ans.to_string();
}

const MOD: usize = 1_000_000_007;

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
            r"3 3",
            "2"
        ),
        (
            r"2 2",
            "0"
        ),
        (
            r"999999 999999",
            "151840682"
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