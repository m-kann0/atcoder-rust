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
    let a: Vec<usize> = (0..(n - 1)).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let s: usize = iterator.next().unwrap().parse().unwrap();

    let mut dp = vec![vec![0_usize; 21]; n];
    dp[1][a[0]] = 1;
    for i in 1..(n - 1) {
        for j in 0..=20 {
            // if j + a[i] <= 20 {
            //     dp[i + 1][j + a[i]] += dp[i][j];
            // }
            // if j >= a[i] {
            //     dp[i + 1][j - a[i]] += dp[i][j];
            // }
            if j >= a[i] {
                dp[i + 1][j] += dp[i][j - a[i]];
            }
            if j + a[i] <= 20 {
                dp[i + 1][j] += dp[i][j + a[i]];
            }
        }
    }
    dp[n - 1][s].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"11
8 3 2 4 8 7 2 4 0 8 8",
            "10"
        ),
        (
            r"40
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 0 0 1 1",
            "7069052760"
        ),
        (
            r"100
0 7 2 8 4 8 3 9 4 9 7 9 0 8 4 7 9 0 1 5 3 4 3 6 9 1 9 3 0 4 7 1 3 8 6 1 8 2 4 4 3 1 3 2 7 9 6 7 6 0 0 7 0 8 8 2 8 6 4 2 7 1 6 7 0 1 6 9 6 7 9 9 3 9 4 6 3 4 1 5 5 1 3 2 6 7 1 9 4 3 8 0 4 8 7 4 7 8 1 6",
            "671013170798012928"
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