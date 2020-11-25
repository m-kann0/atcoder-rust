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

    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let t: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut dp = vec![vec![0_usize; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }

    let mut result = Vec::new();
    let mut i = s.len();
    let mut j = t.len();
    while dp[i][j] > 0 {
        if dp[i - 1][j - 1] == dp[i][j] {
            i -= 1;
            j -= 1;
        } else if dp[i][j - 1] == dp[i][j] {
            j -= 1;
        } else if dp[i - 1][j] == dp[i][j] {
            i -= 1;
        } else {
            result.push(s[i - 1]);
            i -= 1;
            j -= 1;
        }
    }
    result.reverse();
    result.iter().collect::<String>()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"axyb
abyxb",
            "axb"
        ),
        (
            r"aa
xayaz",
            "aa"
        ),
        (
            r"a
z",
            ""
        ),
        (
            r"abracadabra
avadakedavra",
            "aaadara"
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