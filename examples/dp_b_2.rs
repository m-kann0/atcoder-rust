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
    let h: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const INF: isize = 1_000_000_000_000;
    let mut dp = vec![INF; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=k {
            if i >= j {
                dp[i] = min(dp[i], dp[i - j] + (h[i] - h[i - j]).abs());
            }
        }
    }
    dp[n - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
10 30 40 50 20",
            "30"
        ),
        (
            r"3 1
10 20 10",
            "20"
        ),
        (
            r"2 100
10 10",
            "0"
        ),
        (
            r"10 4
40 10 20 70 80 10 20 70 80 60",
            "40"
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