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

    let mut ans = 0;
    for i in 0..s.len() {
        let mut j = i;
        while j < s.len() && (s[j] == 'A' || s[j] == 'C' || s[j] == 'G' || s[j] == 'T') {
            j += 1;
        }
        ans = max(ans, j - i);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ATCODER",
            "3"
        ),
        (
            r"HATAGAYA",
            "5"
        ),
        (
            r"SHINJUKU",
            "0"
        ),
        (
            r"SHINJUKUA",
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