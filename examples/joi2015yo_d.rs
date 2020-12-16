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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let d: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let c: Vec<usize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const INF: usize = std::usize::MAX;
    let mut dp = vec![vec![INF; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = 0;
    }

    for i in 0..m {
        for j in 0..=n {
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
        for j in 0..n {
            if dp[i][j] != INF {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + c[i] * d[j]);
            }
        }
    }

    dp[m][n].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 5
10
25
15
50
30
15
40
30",
            "1125"
        ),
        (
            r"2 6
99
20
490
612
515
131
931
1000",
            "31589"
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