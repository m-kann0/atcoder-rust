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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    // eprintln!("a = {:?}", a);
    // eprintln!("s = {:?}", s);

    const INF: usize = 1_000_000_000_000_000_000;
    let mut dp = vec![vec![INF; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for k in 1..n {
        for l in 0..n {
            let r = l + k;
            if r >= n {
                continue;
            }
            for i in 0..r {
                dp[l][r] = min(dp[l][r], dp[l][i] + dp[i + 1][r] + s[r + 1] - s[l]);
            }
        }
    }
    dp[0][n - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
10 20 30 40",
            "190"
        ),
        (
            r"5
10 10 10 10 10",
            "120"
        ),
        (
            r"3
1000000000 1000000000 1000000000",
            "5000000000"
        ),
        (
            r"6
7 6 8 6 1 1",
            "68"
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