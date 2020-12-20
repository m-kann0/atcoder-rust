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

    let mut s: Vec<char> = iterator.next().unwrap().chars().collect();
    let x: usize = iterator.next().unwrap().parse().unwrap();

    while s[0].is_digit(10) {
        s.remove(0);
    }

    const INF: usize = 1_000_000_000_000_000_000;
    let mut lens = vec![INF; s.len()];
    lens[0] = 1;
    for i in 1..s.len() {
        if s[i].is_digit(10) {
            let d = s[i].to_digit(10).unwrap();
            lens[i] = lens[i - 1] * (d + 1) as usize;
        } else {
            lens[i] = lens[i - 1] + 1;
        }
        if lens[i] > 1_000_000_000_000_000 {
            break;
        }
    }

    // eprintln!();
    // eprintln!("s = {:?}", s);
    // eprintln!("lens = {:?}", lens);

    let ans = rec(x, &s, &lens);
    ans.to_string()
}

fn rec(x: usize, s: &Vec<char>, lens: &Vec<usize>) -> char {
    let mut ok = 0 as isize;
    let mut ng = s.len() as isize;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if lens[mid as usize] <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ok = ok as usize;
    // eprintln!("ok = {:?}", ok);
    if lens[ok] == x {
        let mut i = ok;
        while s[i].is_digit(10) {
            i -= 1;
        }
        return s[i];
    }

    let x_ = x - lens[ok];
    rec(x_, s, lens)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"ab2c1
6",
            "b"
        ),
        (
            r"atcoder
6",
            "e"
        ),
        (
            r"a999999999999999
1000000000000000",
            "a"
        ),
        (
            r"31ab2c1
6",
            "b"
        ),
        (
            r"ab2c1
10",
            "a"
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