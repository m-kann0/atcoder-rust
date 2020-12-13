#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::{max, min};

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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let b: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const INF: usize = 1_000_000;
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            if j < m {
                dp[i][j + 1] = min(dp[i][j + 1], dp[i][j] + 1);
            }
            if i < n {
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j] + 1);
            }
            if i < n && j < m {
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j]);
                } else {
                    dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + 1);
                }
            }
        }
    }

    dp[n][m].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3
1 2 1 3
1 3 1",
            "2"
        ),
        (
            r"4 6
1 3 2 4
1 5 2 6 4 3",
            "3"
        ),
        (
            r"5 5
1 1 1 1 1
2 2 2 2 2",
            "5"
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