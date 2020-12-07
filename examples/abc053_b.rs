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

    let mut a: usize = 0;
    for i in 0..s.len() {
        if s[i] == 'A' {
            a = i;
            break;
        }
    }

    let mut z = s.len();
    for i in (0..s.len()).rev() {
        if s[i] == 'Z' {
            z = i;
            break;
        }
    }

    let ans = z - a + 1;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"QWERTYASDFZXCV",
            "5"
        ),
        (
            r"ZABCZ",
            "4"
        ),
        (
            r"HASFJGHOGAKZZFEGA",
            "12"
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