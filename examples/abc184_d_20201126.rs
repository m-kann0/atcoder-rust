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

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();

    let mut dp = vec![vec![vec![None; 105]; 105]; 105];
    let ans = rec(a, b, c, &mut dp);
    ans.to_string()
}

fn rec(i: usize, j: usize, k: usize, dp: &mut Vec<Vec<Vec<Option<f64>>>>) -> f64 {
    if i == 100 || j == 100 || k == 100 {
        return 0.0;
    }
    if let Some(result) = dp[i][j][k] {
        return result;
    }

    let mut result = 0.0;
    result += i as f64 * rec(i + 1, j, k, dp);
    result += j as f64 * rec(i, j + 1, k, dp);
    result += k as f64 * rec(i, j, k + 1, dp);
    result /= (i + j + k) as f64;
    result += 1.0;

    dp[i][j][k] = Some(result);
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"99 99 99",
            "1.000000000"
        ),
        (
            r"98 99 99",
            "1.331081081"
        ),
        (
            r"0 0 1",
            "99.000000000"
        ),
        (
            r"31 41 59",
            "91.835008202"
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