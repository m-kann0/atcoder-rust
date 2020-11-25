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
    let p: Vec<f64> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut dp = vec![vec![0_f64; n + 1]; n + 1];
    dp[0][0] = 1.0;
    for i in 1..=n {
        for j in 0..=i {
            dp[i][j] += dp[i - 1][j] * (1.0 - p[i - 1]);
            if j >= 1 {
                dp[i][j] += dp[i - 1][j - 1] * p[i - 1];
            }
        }
    }

    let mut ans = 0.0;
    for j in 0..=n {
        if j > n / 2 {
            ans += dp[n][j];
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
0.30 0.60 0.80",
            "0.612"
        ),
        (
            r"1
0.50",
            "0.5"
        ),
        (
            r"5
0.42 0.01 0.42 0.99 0.42",
            "0.3821815872"
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