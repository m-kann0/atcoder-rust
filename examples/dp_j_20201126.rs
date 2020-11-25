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
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        if ai == 1 {
            i += 1;
        } else if ai == 2 {
            j += 1;
        } else if ai == 3 {
            k += 1;
        } else {
            unreachable!();
        }
    }

    let mut dp = vec![vec![vec![None; 305]; 305]; 305];
    let ans = rec(n, i, j, k, &mut dp);
    ans.to_string()
}

fn rec(n: usize, i: usize, j: usize, k: usize, dp: &mut Vec<Vec<Vec<Option<f64>>>>) -> f64 {
    if let Some(result) = dp[i][j][k] {
        return result;
    }
    if i == 0 && j == 0 && k == 0 {
        return 0.0;
    }
    let mut result = 0.0;
    if i > 0 {
        result += rec(n, i - 1, j, k, dp) * i as f64 / (i + j + k) as f64;
    }
    if j > 0 {
        result += rec(n, i + 1, j - 1, k, dp) * j as f64 / (i + j + k) as f64;
    }
    if k > 0 {
        result += rec(n, i, j + 1, k - 1, dp) * k as f64 / (i + j + k) as f64;
    }
    result += n as f64 / (i + j + k) as f64;

    dp[i][j][k] = Some(result);
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 1 1",
            "5.5"
        ),
        (
            r"1
3",
            "3"
        ),
        (
            r"2
1 2",
            "4.5"
        ),
        (
            r"10
1 3 2 3 3 2 3 2 1 3",
            "54.48064457488221"
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