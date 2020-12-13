#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let t: isize = iterator.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        let bi: isize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
    }

    let mut now = n;
    let mut prev = 0;
    for i in 0..m {
        now -= a[i] - prev;
        if now <= 0 {
            return "No".to_string();
        }
        now = min(now + b[i] - a[i], n);
        prev = b[i];
    }
    now -= t - prev;
    if now <= 0 {
        return "No".to_string();
    }
    "Yes".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 2 20
9 11
13 17",
            "Yes"
        ),
        (
            r"10 2 20
9 11
13 16",
            "No"
        ),
        (
            r"15 3 30
5 8
15 17
24 27",
            "Yes"
        ),
        (
            r"20 1 30
20 29",
            "No"
        ),
        (
            r"20 1 30
1 10",
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