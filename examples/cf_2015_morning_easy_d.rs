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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut ans: usize = std::usize::MAX;
    for i in 0..n {
        let len_a = i;
        let len_b = n - i;
        let mut dp = vec![vec![0_usize; len_b + 1]; len_a + 1];
        for j in 0..len_a {
            for k in 0..len_b {
                if s[j] == s[i + k] {
                    dp[j + 1][k + 1] = dp[j][k] + 1;
                } else {
                    dp[j + 1][k + 1] = max(dp[j + 1][k], dp[j][k + 1]);
                }
            }
        }
        let now = len_a + len_b - 2 * dp[len_a][len_b];
        ans = min(ans, now);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8
abacbabc",
            "2"
        ),
        (
            r"8
abababab",
            "0"
        ),
        (
            r"5
abcde",
            "5"
        ),
        (
            r"26
codefestivaltwozeroonefive",
            "14"
        ),
        (
            r"8
aaaaaaaa",
            "0"
        ),
        (
            r"7
aaaaaaa",
            "1"
        ),
        (
            r"1
a",
            "1"
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