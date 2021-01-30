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

    let mut a: usize = iterator.next().unwrap().parse().unwrap();
    let mut b: usize = iterator.next().unwrap().parse().unwrap();
    let mut c: usize = iterator.next().unwrap().parse().unwrap();

    if a > b {
        return "Takahashi".to_string();
    }

    if a < b {
        return "Aoki".to_string();
    }

    if c == 0 {
        return "Aoki".to_string();
    }
    "Takahashi".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 1 0",
            "Takahashi"
        ),
        (
            r"2 2 0",
            "Aoki"
        ),
        (
            r"2 2 1",
            "Takahashi"
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