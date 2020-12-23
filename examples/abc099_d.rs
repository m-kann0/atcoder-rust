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
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let mut d = vec![vec![0_usize; c]; c];
    for i in 0..c {
        for j in 0..c {
            d[i][j] = iterator.next().unwrap().parse().unwrap();
        }
    }
    let mut grid = vec![vec![0_usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = iterator.next().unwrap().parse().unwrap();
            grid[i][j] -= 1;
        }
    }

    let mut x = vec![vec![0; 30]; 3];
    for i in 0..n {
        for j in 0..n {
            x[(i + j) % 3][grid[i][j]] += 1;
        }
    }

    let mut ans: usize = std::usize::MAX;
    for i in 0..c {
        for j in 0..c {
            for k in 0..c {
                if i == j || j == k || k == i {
                    continue;
                }
                let mut now: usize = 0;
                let to = vec![i, j, k];
                for s in 0..3 {
                    for from in 0..c {
                        now += d[from][to[s]] * x[s][from]
                    }
                }
                ans = min(ans, now);
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3
0 1 1
1 0 1
1 4 0
1 2
3 3",
            "3"
        ),
        (
            r"4 3
0 12 71
81 0 53
14 92 0
1 1 2 1
2 1 1 2
2 2 1 3
1 1 2 2",
            "428"
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