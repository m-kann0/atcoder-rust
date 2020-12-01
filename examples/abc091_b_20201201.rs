#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut map = HashMap::new();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let s = iterator.next().unwrap();
        *map.entry(s).or_insert(0) += 1;
    }
    let m: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..m {
        let t = iterator.next().unwrap();
        *map.entry(t).or_insert(0) -= 1;
    }

    let mut ans = 0;
    for (_, v) in map {
        ans = max(ans, v);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
apple
orange
apple
1
grape",
            "2"
        ),
        (
            r"3
apple
orange
apple
5
apple
apple
apple
apple
apple",
            "1"
        ),
        (
            r"1
voldemort
10
voldemort
voldemort
voldemort
voldemort
voldemort
voldemort
voldemort
voldemort
voldemort
voldemort",
            "0"
        ),
        (
            r"6
red
red
blue
yellow
yellow
red
5
red
red
yellow
green
blue",
            "1"
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