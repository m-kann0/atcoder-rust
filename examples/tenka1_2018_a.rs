use std::io::Read;
use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: String = iterator.next().unwrap().to_string();

    if s.len() <= 2 {
        return s;
    }

    return s.chars().rev().join("");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"abc",
            "cba"
        ),
        (
            r"ac",
            "ac"
        ),
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
