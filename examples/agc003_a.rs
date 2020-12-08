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

    let contains_n = s.contains(&'N');
    let contains_w = s.contains(&'W');
    let contains_s = s.contains(&'S');
    let contains_e = s.contains(&'E');

    if contains_n ^ contains_s {
        return "No".to_string();
    }
    if contains_w ^ contains_e {
        return "No".to_string();
    }

    "Yes".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"SENW",
            "Yes"
        ),
        (
            r"NSNNSNSN",
            "Yes"
        ),
        (
            r"NNEW",
            "No"
        ),
        (
            r"W",
            "No"
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