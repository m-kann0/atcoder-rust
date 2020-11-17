#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut s = vec![0; n + 1];
    for i in 1..=n {
        s[i] = s[i - 1] + a[i - 1];
    }

    if a.iter().sum::<usize>() == k {
        return "1".to_string();
    }

    const INF: usize = 1_000_000_005;
    let mut dp = vec![vec![INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        dp[i][0] = 0;
        dp[i][1] = 1;
        for j in 2..=i {
            // let mut ng: isize = 0;
            // let mut ok: isize = (a[i - 1] * dp[i - 1][j - 1] + 1) as isize;
            // while (ok - ng).abs() > 1 {
            //     let mid = (ok + ng) / 2;
            //     if a[i - 1] * dp[i - 1][j - 1] < s[i - 1] * mid as usize {
            //         ok = mid;
            //     } else {
            //         ng = mid;
            //     }
            // }
            // let ok = ok as usize;
            // if ok <= a[i - 1] {
            //     dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - 1] + ok);
            // } else {
            //     dp[i][j] = dp[i - 1][j];
            // }
            // 二分探索不要だった
            let x = a[i - 1] * dp[i - 1][j - 1] / s[i - 1] + 1;
            dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - 1] + x);
        }
    }
    let mut ans = 0;
    for j in 0..=n {
        if dp[n][j] <= k {
            ans = j;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 7
2
3
7
4
9",
            "3"
        ),
        (
            r"3 5
1
2
2",
            "1"
        ),
        (
            r"2 4
2
10",
            "1"
        ),
        (
            r"10 12
2
8
3
5
10
5
2
9
19
22",
            "7"
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