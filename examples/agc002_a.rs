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

    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();

    if a > 0 {
        return "Positive".to_string();
    }

    if b < 0 {
        return if (a.abs() - b.abs()) % 2 == 0 {
            "Negative".to_string()
        } else {
            "Positive".to_string()
        };
    }

    return "Zero".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (r"1 3", "Positive"),
        (r"-3 -1", "Negative"),
        (r"-1 1", "Zero"),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
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