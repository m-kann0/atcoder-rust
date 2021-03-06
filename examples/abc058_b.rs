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

    let o: Vec<char> = iterator.next().unwrap().chars().collect();
    let e: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut result = String::new();
    for i in 0..e.len() {
        result.push(o[i]);
        result.push(e[i]);
    }

    if o.len() > e.len() {
        result.push(o[o.len() - 1]);
    }

    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"xyz
abc",
            "xaybzc"
        ),
        (
            r"atcoderbeginnercontest
atcoderregularcontest",
            "aattccooddeerrbreeggiunlnaerrccoonntteesstt"
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