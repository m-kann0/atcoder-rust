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

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();

    let mut used = vec![false; b];
    let mut x = a % b;
    while !used[x] {
        if x == c {
            return "YES".to_string();
        }
        used[x] = true;
        x += a;
        x %= b;
    }
    "NO".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7 5 1",
            "YES"
        ),
        (
            r"2 2 1",
            "NO"
        ),
        (
            r"1 100 97",
            "YES"
        ),
        (
            r"40 98 58",
            "YES"
        ),
        (
            r"77 42 36",
            "NO"
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