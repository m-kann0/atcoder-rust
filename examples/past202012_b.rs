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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut t = vec![];
    for i in 0..n {
        for j in (0..t.len()).rev() {
            if t[j] == s[i] {
                t.remove(j);
            }
        }
        t.push(s[i]);
    }
    t.iter().collect::<String>()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
aba",
            "ba"
        ),
        (
            r"7
sptaast",
            "past"
        ),
        (
            r"30
ryfoxchyvfmsewlwpoyvhdjkbvdjsa",
            "rxcfmelwpoyhkbvdjsa"
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