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

    let H: usize = iterator.next().unwrap().parse().unwrap();
    let N: usize = iterator.next().unwrap().parse().unwrap();
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        A.push(a);
        B.push(b);
    }

    const INF: usize = 1_000_000_000;
    let mut dp = vec![vec![INF; H + 1]; N + 1];
    dp[0][0] = 0;
    for i in 0..N {
        for j in 0..=H {
            dp[i + 1][j] = dp[i][j];
            if j < A[i] {
                dp[i + 1][j] = min(dp[i + 1][j], B[i]);
            } else {
                dp[i + 1][j] = min(dp[i + 1][j], dp[i + 1][j - A[i]] + B[i]);
            }
        }
    }
    dp[N][H].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"9 3
8 3
4 2
2 1",
            "4"
        ),
        (
            r"100 6
1 1
2 3
3 9
4 27
5 81
6 243",
            "100"
        ),
        (
            r"9999 10
540 7550
691 9680
700 9790
510 7150
415 5818
551 7712
587 8227
619 8671
588 8228
176 2461",
            "139815"
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
