#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::{min, max};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();

    const INF: isize = 1_000_000_000_000_000;
    let mut s = vec![-INF];
    let mut t = vec![-INF];
    for _ in 0..a {
        s.push(iterator.next().unwrap().parse().unwrap());
    }
    for _ in 0..b {
        t.push(iterator.next().unwrap().parse().unwrap());
    }
    s.push(INF);
    t.push(INF);

    let mut result = String::new();
    for _ in 0..q {
        let x: isize = iterator.next().unwrap().parse().unwrap();

        let i = binary_search(x, &s);
        let j = binary_search(x, &t);

        // left left
        let mut now = max(x - s[i], x - t[j]);
        // right right
        now = min(now, max(s[i + 1] - x, t[j + 1] - x));
        // left right
        now = min(now, (x - s[i]) + (t[j + 1] - x) + min(x - s[i], t[j + 1] - x));
        // right left
        now = min(now, (s[i + 1] - x) + (x - t[j]) + min(s[i + 1] - x, x - t[j]));

        result.push_str(&format!("{}\n", now));
    }
    result
}

fn binary_search(x: isize, v: &Vec<isize>) -> usize {
    let mut ok: isize = 0;
    let mut ng: isize = v.len() as isize;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if v[mid as usize] < x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok as usize
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 4
100
600
400
900
1000
150
2000
899
799",
            "350
1400
301
399"
        ),
        (
            r"1 1 3
1
10000000000
2
9999999999
5000000000",
            "10000000000
10000000000
14999999998"
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