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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let l: usize = iterator.next().unwrap().parse().unwrap();
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const INF: usize = 1_000_000_000;
    let mut dp = vec![vec![INF; m]; 2];
    dp[0][0] = 0;
    for i in 0..n {
        let prev = i % 2;
        let now = (i + 1) % 2;
        for j in 0..m {
            let sho = a[i] / m;
            let amari = a[i] % m;
            if j >= amari {
                dp[now][j] = min(dp[prev][j], dp[prev][j - amari] + sho);
            } else {
                dp[now][j] = min(dp[prev][j], dp[prev][j + m - amari] + sho + 1);
            }
        }
    }
    if dp[n % 2][l] < x {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 11 7 5
1 4 5 8 9",
            "Yes"
        ),
        (
            r"5 5 3 2
1 4 5 9 12",
            "No"
        ),
        (
            r"5 10 3 100
1 4 7 10 14",
            "No"
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