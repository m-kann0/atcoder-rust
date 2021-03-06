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

    if s.len() % 2 != 0 {
        panic!();
    }

    let mut n = s.len() - 2;
    while n > 0 {
        let mut is_ok = true;
        for i in 0..(n / 2) {
            if s[i] != s[n / 2 + i] {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            return n.to_string();
        }
        n -= 2;
    }
    panic!();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"abaababaab",
            "6"
        ),
        (
            r"xxxx",
            "2"
        ),
        (
            r"abcabcabcabc",
            "6"
        ),
        (
            r"akasakaakasakasakaakas",
            "14"
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