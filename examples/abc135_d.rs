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

    let S: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut dp = vec![vec![vec![0_usize; 13]; 10]; S.len()];
    let mut x = 1;
    for i in (0..S.len()).rev() {
        if i == S.len() - 1 {
            if S[i] == '?' {
                for j in 0..=9 {
                    dp[i][j][j] = 1;
                }
            } else {
                let j = S[i].to_digit(10).unwrap() as usize;
                dp[i][j][j] = 1;
            }
        } else {
            for j in 0..=9 {
                if S[i] == '?' || S[i].to_digit(10).unwrap() as usize == j {
                    for k in 0..=9 {
                        for l in 0..=12 {
                            dp[i][j][(j * x + l) % 13] += dp[i + 1][k][l];
                            dp[i][j][(j * x + l) % 13] %= 1_000_000_007;
                        }
                    }
                }
            }
        }
        x *= 10;
        x %= 13;
    }
    let mut ans = 0;
    for j in 0..=9 {
        ans += dp[0][j][5];
        ans %= 1_000_000_007;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"??2??5",
            "768"
        ),
        (
            r"?44",
            "1"
        ),
        (
            r"7?4",
            "0"
        ),
        (
            r"?6?42???8??2??06243????9??3???7258??5??7???????774????4?1??17???9?5?70???76???",
            "153716888"
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