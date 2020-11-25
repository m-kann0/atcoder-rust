#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let N: usize = iterator.next().unwrap().parse().unwrap();
    let W: usize = iterator.next().unwrap().parse().unwrap();
    let mut w: Vec<usize> = Vec::new();
    let mut v: Vec<usize> = Vec::new();
    for _ in 0..N {
        let wi: usize = iterator.next().unwrap().parse().unwrap();
        let vi: usize = iterator.next().unwrap().parse().unwrap();
        w.push(wi);
        v.push(vi);
    }

    let mut dp = vec![vec![0; W + 1]; N + 1];
    for i in 0..N {
        for j in 0..=W {
            if j >= w[i] {
                dp[i + 1][j] = max(dp[i][j], dp[i][j - w[i]] + v[i]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    dp[N][W].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 8
3 30
4 50
5 60",
            "90"
        ),
        (
            r"5 5
1 1000000000
1 1000000000
1 1000000000
1 1000000000
1 1000000000",
            "5000000000"
        ),
        (
            r"6 15
6 5
5 6
6 4
6 6
3 5
7 2",
            "17"
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