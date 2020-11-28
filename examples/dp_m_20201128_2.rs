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

    const MOD: usize = 1_000_000_007;
    let mut dp = vec![vec![0_usize; K + 1]; N + 1];
    dp[0][0] = 1;
    for i in 1..=N {
        let mut s = vec![0; K + 2];
        for j in 1..=(K + 1) {
            s[j] = (s[j - 1] + dp[i - 1][j - 1]) % MOD;
        }
        for j in 0..=K {
            let l = if j >= a[i - 1] {
                j - a[i - 1]
            } else {
                0
            };
            let r = j + 1;
            dp[i][j] = (s[r] - s[l] + MOD) % MOD;
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