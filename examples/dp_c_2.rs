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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a = vec![vec![0; 3]; n];
    for i in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        a[i][0] = ai;
        a[i][1] = bi;
        a[i][2] = ci;
    }

    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 1..=n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }
                dp[i][j] = max(dp[i][j], dp[i - 1][k] + a[i - 1][j]);
            }
        }
    }
    dp[n].iter().max().unwrap().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
10 40 70
20 50 80
30 60 90",
            "210"
        ),
        (
            r"1
100 10 1",
            "100"
        ),
        (
            r"7
6 7 8
8 8 3
2 5 2
7 8 6
4 6 8
2 3 4
7 5 1",
            "46"
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