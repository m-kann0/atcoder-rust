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
    for i in 1..=n {
        s[i] = a[i - 1] + s[i - 1];
    }

    let mut dp = vec![vec![None; n + 1]; n + 1];
    let result = rec(0, n - 1, &mut dp, &s);
    result.to_string()
}

fn rec(l: usize, r: usize, dp: &mut Vec<Vec<Option<usize>>>, s: &Vec<usize>) -> usize {
    if let Some(result) = dp[l][r] {
        return result;
    }
    if l == r {
        dp[l][r] = Some(0);
        return 0;
    }
    let mut result = std::usize::MAX;
    for m in l..r {
        result = min(result, rec(l, m, dp, s) + rec(m + 1, r, dp, s) + s[r + 1] - s[l]);
    }
    dp[l][r] = Some(result);
    return result;
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