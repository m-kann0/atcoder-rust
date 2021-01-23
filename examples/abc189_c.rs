#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashSet;
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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut s = HashSet::new();
    for i in 0..n {
        s.insert(a[i]);
    }

    let mut ans = 0;
    for &si in &s {
        let mut cur = false;
        let mut l = 0;
        let mut r = 0;
        for i in 0..n {
            if a[i] >= si {
                if !cur {
                    cur = true;
                    l = i;
                }
                r = i;
            } else {
                cur = false;
                ans = max(ans, si * (r - l + 1));
            }
        }
        if cur {
            ans = max(ans, si * (r - l + 1));
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
2 4 4 9 4 9",
            "20"
        ),
        (
            r"6
200 4 4 9 4 9",
            "200"
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