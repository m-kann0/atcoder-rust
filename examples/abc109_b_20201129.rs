#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut w = Vec::new();
    for _ in 0..n {
        let wi = iterator.next().unwrap().chars().collect::<Vec<char>>();
        w.push(wi);
    }

    let mut prev = w[0].clone();
    let mut history = HashSet::new();
    history.insert(w[0].iter().collect::<String>());
    for i in 1..n {
        let current = w[i].clone();
        if history.contains(&w[i].iter().collect::<String>()) {
            return "No".to_string();
        }
        if current[0] != prev[prev.len() - 1] {
            return "No".to_string();
        }
        prev = current;
        history.insert(w[i].iter().collect::<String>());
    }
    "Yes".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
hoge
english
hoge
enigma",
            "No"
        ),
        (
            r"9
basic
c
cpp
php
python
nadesico
ocaml
lua
assembly",
            "Yes"
        ),
        (
            r"8
a
aa
aaa
aaaa
aaaaa
aaaaaa
aaa
aaaaaaa",
            "No"
        ),
        (
            r"3
abc
arc
agc",
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