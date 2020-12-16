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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut F = vec![-1; n];
    for _ in 0..k {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: isize = iterator.next().unwrap().parse().unwrap();
        F[a - 1] = b - 1;
    }

    let mut dp = vec![vec![vec![0_usize; 3]; 3]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for a in 0..3 {
            for b in 0..3 {
                let cs = if F[i] == -1 {
                    vec![0, 1, 2]
                } else {
                    vec![F[i] as usize]
                };
                for c in cs {
                    if i >= 2 && a == b && b == c {
                        continue;
                    }
                    dp[i + 1][b][c] += dp[i][a][b];
                    dp[i + 1][b][c] %= 10_000;
                }
            }
        }
    }
    let mut ans = 0;
    for a in 0..3 {
        for b in 0..3 {
            ans += dp[n][a][b];
            ans %= 10_000;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
3 1
1 1
4 2",
            "6"
        ),
        (
            r"20 5
10 2
4 3
12 1
13 2
9 1",
            "2640"
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