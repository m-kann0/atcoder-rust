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

    max(s(a), s(b)).to_string()
}

fn s(mut n: usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }
    ans
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"123 234",
            "9"
        ),
        (
            r"593 953",
            "17"
        ),
        (
            r"100 999",
            "27"
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