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

    let d: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let t: Vec<usize> = (0..d).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: isize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    let mut dp = vec![vec![0; n]; d];
    for i in 0..d {
        for j in 0..n {
            if a[j] <= t[i] && t[i] <= b[j] {
                if i == 0 {
                    dp[i][j] = 0;
                    continue;
                }
                for k in 0..n {
                    if dp[i - 1][k] == -1 {
                        continue;
                    }
                    dp[i][j] = max(dp[i][j], dp[i - 1][k] + (c[j] - c[k]).abs());
                }
            } else {
                dp[i][j] = -1;
            }
        }
    }

    let mut ans = 0;
    for j in 0..n {
        ans = max(ans, dp[d - 1][j]);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
31
27
35
20 25 30
23 29 90
21 35 60
28 33 40",
            "80"
        ),
        (
            r"5 2
26
28
32
29
34
30 35 0
25 30 100",
            "300"
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