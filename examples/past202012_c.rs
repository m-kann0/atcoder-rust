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

    let chars: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let mut n: usize = iterator.next().unwrap().parse().unwrap();
    if n == 0 {
        return "0".to_string();
    }
    let mut result = Vec::new();
    while n > 0 {
        result.push(chars[n % 36]);
        n /= 36;
    }
    result.iter().rev().collect::<String>()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"123",
            "3F"
        ),
        (
            r"2304",
            "1S0"
        ),
        (
            r"0",
            "0"
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