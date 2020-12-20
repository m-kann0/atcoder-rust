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
    let mut prev = '-';
    let mut count = 0;
    for i in 0..5 {
        if s[i] == prev {
            count += 1;
        } else {
            prev = s[i];
            count = 1;
        }
        // eprintln!("prev = {:?}", prev);
        // eprintln!("count = {:?}", count);
        if count >= 3 {
            return prev.to_string();
        }
    }
    "draw".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"xooox",
            "o"
        ),
        (
            r"xxxxx",
            "x"
        ),
        (
            r"xoxxo",
            "draw"
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