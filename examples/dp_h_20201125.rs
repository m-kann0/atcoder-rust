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

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let mut grid = Vec::new();
    for _ in 0..h {
        let line = iterator.next().unwrap().chars().collect::<Vec<char>>();
        grid.push(line);
    }

    const MOD: usize = 1_000_000_007;
    let mut dp = vec![vec![0_usize; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                continue;
            }
            if j >= 1 {
                dp[i][j] += dp[i][j - 1];
                dp[i][j] %= MOD;
            }
            if i >= 1 {
                dp[i][j] += dp[i - 1][j];
                dp[i][j] %= MOD;
            }
        }
    }
    dp[h - 1][w - 1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
...#
.#..
....",
            "3"
        ),
        (
            r"5 2
..
#.
..
.#
..",
            "0"
        ),
        (
            r"5 5
..#..
.....
#...#
.....
..#..",
            "24"
        ),
        (
            r"20 20
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................",
            "345263555"
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