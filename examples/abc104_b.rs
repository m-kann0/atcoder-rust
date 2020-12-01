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

    if s[0] != 'A' {
        return "WA".to_string();
    }

    let mut ci = 0;
    for i in 2..=s.len() - 2 {
        if s[i] == 'C' {
            ci = i;
            break;
        }
    }

    if ci == 0 {
        return "WA".to_string();
    }

    for i in 1..s.len() {
        if i == ci {
            continue;
        }
        if !s[i].is_lowercase() {
            return "WA".to_string();
        }
    }

    "AC".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"AtCoder",
            "AC"
        ),
        (
            r"ACoder",
            "WA"
        ),
        (
            r"AcycliC",
            "WA"
        ),
        (
            r"AtCoCo",
            "WA"
        ),
        (
            r"Atcoder",
            "WA"
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