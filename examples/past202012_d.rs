#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::max;
use std::collections::{VecDeque, HashMap};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut s = Vec::new();
    for _ in 0..n {
        let si: String = iterator.next().unwrap().to_string();
        s.push(si);
    }

    let mut map = HashMap::new();
    for i in 0..n {
        let len = remove_lead_zero(&s[i]).len();
        map.entry(len).or_insert(Vec::new()).push(s[i].clone());
    }

    eprintln!("map = {:?}", map);

    let mut result = String::new();
    for i in 0..=100_000 {
        if let Some(list) = map.get(&i) {
            let mut mx_len = 0;
            for s in list {
                mx_len = max(mx_len, s.len());
            }
            let mut p = Vec::new();
            for s in list {
                let t = pad_zero(mx_len, s.clone());
                let k = if is_all_zero(&s) {
                    repeat(mx_len - s.len())
                } else {
                    s.clone()
                };
                p.push((t, k, s.clone()));
            }
            p.sort();

            for (_t, _k, s) in p.iter() {
                result.push_str(&format!("{}\n", s));
            }
        }
    }
    result
}

fn remove_lead_zero(s: &str) -> String {
    let mut result: VecDeque<char> = s.chars().collect();
    while !result.is_empty() && *result.front().unwrap() == '0' {
        result.pop_front();
    }
    result.iter().collect::<String>()
}

fn is_all_zero(s: &str) -> bool {
    for c in s.chars() {
        if c != '0' {
            return false;
        }
    }
    true
}

fn pad_zero(len: usize, s: String) -> String {
    let lead = repeat(len - s.len());
    let result = lead + &s;
    result
}

fn repeat(n: usize) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push('0');
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2
1
01
1
0010",
            "01
1
1
2
0010"
        ),
        (
            r"6
1111111111111111111111
00011111111111111111111
000000111111111111111111
0000000001111111111111111
00000000000011111111111111
000000000000000111111111111",
            "000000000000000111111111111
00000000000011111111111111
0000000001111111111111111
000000111111111111111111
00011111111111111111111
1111111111111111111111"
        ),
        (
            r"2
0
00",
            "00
0"
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