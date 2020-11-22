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
    let t: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ts = Vec::new();
    let n2 = n / 2;
    for i in 0..(1 << n2) {
        let mut st = 0;
        for j in 0..n2 {
            if ((i >> j) & 1) > 0 {
                st += a[j];
            }
        }
        ts.push(st);
    }
    ts.sort();
    if ts.is_empty() {
        ts.push(0);
    }

    let mut res = 0;
    for i in 0..(1 << (n - n2)) {
        let mut st = 0;
        for j in 0..(n - n2) {
            if ((i >> j) & 1) > 0 {
                st += a[n2 + j];
            }
        }
        if st > t {
            continue;
        }

        let mut ok: isize = 0;
        let mut ng: isize = ts.len() as isize;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if st + ts[mid as usize] <= t {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        res = max(res, st + ts[ok as usize]);
    }
    res.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 17
2 3 5 7 11",
            "17"
        ),
        (
            r"6 100
1 2 7 5 8 10",
            "33"
        ),
        (
            r"6 100
101 102 103 104 105 106",
            "0"
        ),
        (
            r"7 273599681
6706927 91566569 89131517 71069699 75200339 98298649 92857057",
            "273555143"
        ),
        (
            r"1 17
11",
            "11"
        ),
        (
            r"1 10
11",
            "0"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
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