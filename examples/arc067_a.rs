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

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut factors: HashMap<usize, usize> = HashMap::new();
    for i in 2..(n + 1) {
        let f = prime_factorization(i);
        for x in f {
            *factors.entry(x.0).or_insert(0) += x.1;
        }
    }

    const MOD: usize = 1_000_000_007;
    let mut ans: usize = 1;
    for x in factors {
        ans = ans * (x.1 + 1) % MOD;
    }
    return ans.to_string();
}

fn prime_factorization(mut n: usize) -> HashMap<usize, usize> {
    let mut result: HashMap<_, _> = HashMap::new();

    for d in 2.. {
        if d * d > n {
            break;
        }
        while n % d == 0 {
            *result.entry(d).or_insert(0) += 1;
            n /= d;
        }
    }

    if n != 1 {
        result.insert(n, 1);
    }

    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3",
            "4"
        ),
        (
            r"6",
            "30"
        ),
        (
            r"1000",
            "972926972"
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