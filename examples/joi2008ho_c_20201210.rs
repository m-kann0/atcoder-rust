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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut p: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    p.push(0);
    let n = n + 1;

    let mut cd = Vec::new();
    for c in 0..n {
        for d in 0..n {
            cd.push(p[c] + p[d]);
        }
    }
    cd.sort();

    let mut ans = 0;
    for a in 0..n {
        for b in 0..n {
            let ab = p[a] + p[b];
            if ab > m {
                continue;
            }
            let mut ok = 0 as isize;
            let mut ng = cd.len() as isize;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if ab + cd[mid as usize] <= m {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ans = max(ans, ab + cd[ok as usize]);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 50
3
14
15
9",
            "48"
        ),
        (
            r"3 21
16
11
2",
            "20"
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