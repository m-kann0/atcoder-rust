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

    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    for _ in 0..n {
        let mut s: String = iterator.next().unwrap().to_string();
        if s.starts_with("!") {
            s.remove(0);
            s2.insert(s);
        } else {
            s1.insert(s);
        }
    }

    // eprintln!("s1 = {:?}", s1);
    // eprintln!("s2 = {:?}", s2);

    for s in s1 {
        if s2.contains(&s) {
            return s;
        }
    }
    "satisfiable".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
a
!a
b
!c
d
!d",
            "a"
        ),
        (
            r"10
red
red
red
!orange
yellow
!blue
cyan
!green
brown
!gray",
            "satisfiable"
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