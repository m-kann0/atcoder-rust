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

    let N: Vec<usize> = iterator.next().unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let K: usize = iterator.next().unwrap().parse().unwrap();

    // dp[桁数][未満フラグ][0でない数字の個数] := 総数
    let mut dp = vec![vec![vec![0_usize; K + 1]; 2]; N.len() + 1];
    dp[0][0][0] = 1;
    for i in 0..N.len() {
        for j in 0..2 {
            for k in 0..=K {
                let d_max = if j == 1 { 9 } else { N[i] };
                for d in 0..=d_max {
                    let flag = if j == 1 || d < d_max { 1 } else { 0 };
                    let nk = if d == 0 { k } else { k + 1 };
                    if nk <= K {
                        dp[i + 1][flag][nk] += dp[i][j][k];
                    }
                }
            }
        }
    }

    let ans = dp[N.len()][0][K] + dp[N.len()][1][K];
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"100
1",
            "19"
        ),
        (
            r"25
2",
            "14"
        ),
        (
            r"314159
2",
            "937"
        ),
        (
            r"9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
3",
            "117879300"
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