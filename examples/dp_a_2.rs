#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let h: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    // let mut dp = vec![0; n];
    // for i in 1..n {
    //     if i >= 2 {
    //         dp[i] = min(
    //             dp[i - 1] + (h[i] - h[i - 1]).abs(),
    //             dp[i - 2] + (h[i] - h[i - 2]).abs(),
    //         );
    //     } else {
    //         dp[i] = dp[i - 1] + (h[i] - h[i - 1]).abs();
    //     }
    // }
    // dp[n - 1].to_string()

    const INF: isize = 1_000_000_000_000;
    let mut dp = vec![INF; n];
    dp[0] = 0;
    for i in 0..n {
        if i >= 1 {
            dp[i] = min(dp[i], dp[i - 1] + (h[i] - h[i - 1]).abs());
        }
        if i >= 2 {
            dp[i] = min(dp[i], dp[i - 2] + (h[i] - h[i - 2]).abs());
        }
    }
    dp[n - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
10 30 40 20",
            "30"
        ),
        (
            r"2
10 10",
            "0"
        ),
        (
            r"6
30 10 60 10 60 50",
            "40"
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