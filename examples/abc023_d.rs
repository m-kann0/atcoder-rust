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
    let mut h = Vec::new();
    let mut s = Vec::new();
    for _ in 0..n {
        let hi: isize = iterator.next().unwrap().parse().unwrap();
        let si: isize = iterator.next().unwrap().parse().unwrap();
        h.push(hi);
        s.push(si);
    }

    let mut ok: isize = 1_000_000_000_000_000_000;
    let mut ng: isize = 0;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if can_do(n, &h, &s, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok.to_string()
}

fn can_do(n: usize, h: &Vec<isize>, s: &Vec<isize>, x: isize) -> bool {
    let mut a = Vec::new();
    for i in 0..n {
        if h[i] > x {
            return false;
        }
        let ai = (x - h[i]) / s[i];
        a.push(ai);
    }
    a.sort();
    for i in 0..n {
        if a[i] < i as isize {
            return false;
        }
    }
    return true;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
5 6
12 4
14 7
21 2",
            "23"
        ),
        (
            r"6
100 1
100 1
100 1
100 1
100 1
1 30",
            "105"
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