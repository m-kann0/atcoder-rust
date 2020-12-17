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
    let mut a = vec![0; 2 * n];
    for i in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        a[i] = ai;
        a[n + i] = ai;
    }

    let mut dp = vec![vec![0; n * 2]; n * 2];
    if n % 2 == 1 {
        for i in 0..(n * 2) {
            dp[i][i] = a[i];
        }
    }


    for len in 1..n {
        for l in 0..(n * 2) {
            let r = l + len;
            if r >= n * 2 {
                continue;
            }
            if len % 2 != n % 2 {
                dp[l][r] = max(dp[l][r - 1] + a[r], dp[l + 1][r] + a[l]);
            } else {
                dp[l][r] = if a[l] > a[r] {
                    dp[l + 1][r]
                } else {
                    dp[l][r - 1]
                };
            }
        }
        // debug(&dp);
    }

    // debug(&dp);

    let mut ans = 0;
    for l in 0..n {
        let r = l + n - 1;
        ans = max(ans, dp[l][r]);
    }
    ans.to_string()
}

fn debug(dp: &Vec<Vec<usize>>) {
    eprintln!();
    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            eprint!("{:3} ", dp[i][j]);
        }
        eprintln!();
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2
8
1
10
9",
            "18"
        ),
        (
            r"8
1
10
4
5
6
2
9
3",
            "26"
        ),
        (
            r"15
182243672
10074562
977552215
122668426
685444213
3784162
463324752
560071245
134465220
21447865
654556327
183481051
20041805
405079805
564327789",
            "3600242976"
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