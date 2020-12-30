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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut ans: usize = 0;
    for i in 0..n {
        let mut count: usize = 0;
        for c in ('a' as u8)..=('z' as u8) {
            if contains(&s, 0, i, c as char) && contains(&s, i, n, c as char) {
                count += 1;
            }
        }
        ans = max(ans, count);
    }
    ans.to_string()
}

fn contains(s: &Vec<char>, begin: usize, end: usize, target: char) -> bool {
    for i in begin..end {
        if s[i] == target {
            return true;
        }
    }
    false
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
aabbca",
            "2"
        ),
        (
            r"10
aaaaaaaaaa",
            "1"
        ),
        (
            r"45
tgxgdqkyjzhyputjjtllptdfxocrylqfqjynmfbfucbir",
            "9"
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