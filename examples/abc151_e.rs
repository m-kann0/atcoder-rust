#![allow(non_snake_case)]

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    a.sort();

    const MOD: isize = 1_000_000_007;
    let mc = mod_comb::ModComb::init(1_000_000_007, n + 5);

    let mut ans: isize = 0;
    for i in 0..n {
        // eprintln!("i = {:?}", i);
        // eprintln!("a[i] = {:?}", a[i]);
        // 最小値として選ばれる回数
        // if n - i - 1 >= k - 1 {
            let x = mc.calc(n - i - 1, k - 1);
            // eprintln!("as min = {:?}", x);
            ans = (ans - (a[i] * x as isize) % MOD) % MOD;
        // }
        // 最大値として選ばれる回数
        // if i >= k - 1 {
            let x = mc.calc(i, k - 1);
            // eprintln!("as max = {:?}", x);
            ans = (ans + (a[i] * x as isize) % MOD) % MOD;
        // }
    }

    ans = (ans + MOD) % MOD;
    ans.to_string()
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
            r"4 2
1 1 3 4",
            "11"
        ),
        (
            r"6 3
10 10 10 -10 -10 -10",
            "360"
        ),
        (
            r"3 1
1 1 1",
            "0"
        ),
        (
            r"10 6
1000000000 1000000000 1000000000 1000000000 1000000000 0 0 0 0 0",
            "999998537"
        ),        (
            r"4 4
1 1 3 4",
            "3"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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