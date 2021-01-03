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

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            return format!("{} {}", i, i + 1);
        }
    }
    for i in 2..s.len() {
        if s[i] == s[i - 2] {
            return format!("{} {}", i - 1, i + 1);
        }
    }

    "-1 -1".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"needed",
            "2 5"
        ),
        (
            r"atcoder",
            "-1 -1"
        ),
        (
            r"nabded",
            "4 6"
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