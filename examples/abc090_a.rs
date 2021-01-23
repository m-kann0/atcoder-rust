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

    let c1: Vec<char> = iterator.next().unwrap().chars().collect();
    let c2: Vec<char> = iterator.next().unwrap().chars().collect();
    let c3: Vec<char> = iterator.next().unwrap().chars().collect();

    format!("{}{}{}", c1[0], c2[1], c3[2])
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ant
obe
rec",
            "abc"
        ),
        (
            r"edu
cat
ion",
            "ean"
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