#![allow(non_snake_case)]

use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let w: usize = iterator.next().unwrap().parse().unwrap();
    let h: usize = iterator.next().unwrap().parse().unwrap();
    let mut grid = vec![vec![2; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            let g: usize = iterator.next().unwrap().parse().unwrap();
            grid[i + 1][j + 1] = g;
        }
    }
    // debug(&grid);
    let h = h + 2;
    let w = w + 2;
    let mut q = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 2 {
                q.push_back((i, j));
            }
        }
    }

    let dh = vec![-1, -1, 0, 0, 1, 1];
    let dw = vec![
        vec![-1, 0, -1, 1, -1, 0],
        vec![0, 1, -1, 1, 0, 1],
    ];

    while let Some((ch, cw)) = q.pop_front() {
        for i in 0..6 {
            let nh = ch as isize + dh[i];
            let nw = cw as isize + dw[ch % 2][i];
            if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                continue;
            }
            let nh = nh as usize;
            let nw = nw as usize;
            if grid[nh][nw] == 0 {
                grid[nh][nw] = 2;
                q.push_back((nh, nw));
            }
        }
    }
    // debug(&grid);

    let mut ans: usize = 0;
    for ch in 0..h {
        for cw in 0..w {
            if grid[ch][cw] != 1 {
                continue;
            }
            for i in 0..6 {
                let nh = ch as isize + dh[i];
                let nw = cw as isize + dw[ch % 2][i];
                if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                    continue;
                }
                let nh = nh as usize;
                let nw = nw as usize;
                if grid[nh][nw] == 2 {
                    ans += 1;
                }
            }
        }
    }
    ans.to_string()
}

fn debug(grid: &Vec<Vec<usize>>) {
    println!();
    for i in 0..grid.len() {
        if i % 2 == 1 {
            print!(" ");
        }
        for j in 0..grid[i].len() {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8 4
0 1 0 1 0 1 1 1
0 1 1 0 0 1 0 0
1 0 1 0 1 1 1 1
0 1 1 0 1 0 1 0",
            "64"
        ),
        (
            r"8 5
0 1 1 1 0 1 1 1
0 1 0 0 1 1 0 0
1 0 0 1 1 1 1 1
0 1 0 1 1 0 1 0
0 1 1 0 1 1 0 0",
            "56"
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