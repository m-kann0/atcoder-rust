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

    let N: usize = iterator.next().unwrap().parse().unwrap();
    let K: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..N).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut dp = vec![vec![0_usize; K + 1]; N + 1];
    for i in 0..=N {
        dp[i][0] = 1;
    }
    const MOD: usize = 1_000_000_007;
    for i in 0..N {
        for j in 1..=K {
            if j >= 1 + a[i] {
                dp[i + 1][j] = (dp[i + 1][j - 1] + dp[i][j] - dp[i][j - 1 - a[i]] + MOD) % MOD;
            } else {
                dp[i + 1][j] = (dp[i + 1][j - 1] + dp[i][j]) % MOD;
            }
        }
    }
    dp[N][K].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
1 2 3",
            "5"
        ),
        (
            r"1 10
9",
            "0"
        ),
        (
            r"2 0
0 0",
            "1"
        ),
        (
            r"4 100000
100000 100000 100000 100000",
            "665683269"
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