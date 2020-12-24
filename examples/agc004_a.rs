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

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();

    if a % 2 == 0 || b % 2 == 0 || c % 2 == 0 {
        return "0".to_string();
    }

    if a == max(a, max(b, c)) {
        return (b * c).to_string();
    }

    if b == max(a, max(b, c)) {
        return (a * c).to_string();
    }

    if c == max(a, max(b, c)) {
        return (a * b).to_string();
    }

    unreachable!();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 3",
            "9"
        ),
        (
            r"2 2 4",
            "0"
        ),
        (
            r"5 3 5",
            "15"
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