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

    let n: Vec<usize> = iterator.next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut ans = 0;
    for i in 0..n.len() {
        ans += n[i];
    }
    ans = max(ans, n[0] - 1 + (n.len() - 1) * 9);
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"100",
            "18"
        ),
        (
            r"9995",
            "35"
        ),
        (
            r"3141592653589793",
            "137"
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