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

    let a = iterator.next().unwrap();
    let b = iterator.next().unwrap();

    if a.len() > b.len() {
        return "GREATER".to_string();
    }
    if a.len() < b.len() {
        return "LESS".to_string();
    }
    if a > b {
        return "GREATER".to_string();
    }
    if a < b {
        return "LESS".to_string();
    }
    "EQUAL".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"36
24",
            "GREATER"
        ),
        (
            r"850
3777",
            "LESS"
        ),
        (
            r"9720246
22516266",
            "LESS"
        ),
        (
            r"123456789012345678901234567890
234567890123456789012345678901",
            "LESS"
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