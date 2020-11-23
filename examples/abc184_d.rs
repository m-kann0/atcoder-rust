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

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();

    let mut dp = vec![vec![vec![0_f64; 101]; 101]; 101];

    for i in (0..=99).rev() {
        for j in (0..=99).rev() {
            for k in (0..=99).rev() {
                if i + j + k == 0 {
                    continue;
                }
                let mut now = 0.0;
                now += dp[i + 1][j][k] * i as f64;
                now += dp[i][j + 1][k] * j as f64;
                now += dp[i][j][k + 1] * k as f64;
                dp[i][j][k] = now / (i + j + k) as f64 + 1_f64;
            }
        }
    }
    dp[a][b][c].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"99 99 99",
            "1.000000000"
        ),
        (
            r"98 99 99",
            "1.331081081"
        ),
        (
            r"0 0 1",
            "99.000000000"
        ),
        (
            r"31 41 59",
            "91.835008202"
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