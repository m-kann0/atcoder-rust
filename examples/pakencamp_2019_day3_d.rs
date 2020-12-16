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
    let mut grid = Vec::new();
    for _ in 0..5 {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        grid.push(line);
    }

    let mut color = vec![vec![0_usize; 3]; n];
    for i in 0..n {
        for j in 0..5 {
            match grid[j][i] {
                'R' => color[i][0] += 1,
                'B' => color[i][1] += 1,
                'W' => color[i][2] += 1,
                _ => {},
            }
        }
    }

    const INF: usize = std::usize::MAX;
    let mut dp = vec![vec![INF; 3]; n + 1];
    for i in 0..3 {
        dp[0][i] = 0;
    }

    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k { continue; }
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][k] + 5 - color[i][j]);
            }
        }
    }

    let ans = min(dp[n][0], min(dp[n][1], dp[n][2]));
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
B
R
#
W
B",
            "3"
        ),
        (
            r"3
WWR
#RW
BW#
##B
RBR",
            "10"
        ),
        (
            r"8
RRRRRRRR
########
BBBBBBBB
RRRRRRRR
WWWWWWWW",
            "28"
        ),
        (
            r"7
BR#WB#R
RWW#WRB
##WBR#W
WB#B#RW
BRW##BB",
            "21"
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