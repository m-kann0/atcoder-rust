#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashMap;

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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut dp = vec![vec![' '; n]; k + 1];
    for j in 0..n {
        let a = s[j];
        let b = s[(j + 1) % n];
        dp[1][j] = judge(a, b);
    }

    let mut x = 1;
    for i in 2..=k {
        x *= 2;
        x %= n;
        for j in 0..n {
            let a = dp[i - 1][j];
            let b = dp[i - 1][(j + x) % n];
            dp[i][j] = judge(a, b);
        }
    }

    dp[k][0].to_string()

    // let mut memo: HashMap<(usize, usize), char> = HashMap::new();
    // let ans = rec(n, k, &s, 0, &mut memo);
    // ans.to_string()
}

// fn rec(n: usize, k: usize, s: &Vec<char>, i: usize, memo: &mut HashMap<(usize, usize), char>) -> char {
//     if k == 1 {
//         let a = s[i];
//         let b = s[(i + 1) % n];
//         return judge(a, b);
//     }
//     if let Some(result) = memo.get(&(k, i)) {
//         return *result;
//     }
//
//     let a = rec(n, k - 1, s, i, memo);
//     let mut j = 1;
//     for _ in 0..(k - 1) {
//         j *= 2;
//         j %= n;
//     }
//     j += 1 + i;
//     j %= n;
//     let b = rec(n, k - 1, s, j, memo);
//     return judge(a, b);
// }

fn judge(a: char, b: char) -> char {
    if a == b {
        return a;
    }
    if a == 'R' {
        return if b == 'S' {
            a
        } else {
            b
        }
    } else if a == 'S' {
        return if b == 'P' {
            a
        } else {
            b
        }
    } else if a == 'P' {
        return if b == 'R' {
            a
        } else {
            b
        }
    } else {
        unreachable!();
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
RPS",
            "P"
        ),
        (
            r"11 1
RPSSPRSPPRS",
            "P"
        ),
        (
            r"1 100
S",
            "S"
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