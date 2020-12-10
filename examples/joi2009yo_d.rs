#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::max;

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
    let mut grid = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            let a: usize = iterator.next().unwrap().parse().unwrap();
            grid[i][j] = a == 1;
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = max(ans, rec(i, j, &mut grid));
        }
    }
    ans.to_string()
}

fn rec(ch: usize, cw: usize, grid: &mut Vec<Vec<bool>>) -> usize {
    if !grid[ch][cw] {
        return 0;
    }
    grid[ch][cw] = false;
    let dh = vec![1, -1, 0, 0];
    let dw = vec![0, 0, 1, -1];
    let mut m = 0;
    for i in 0..4 {
        let nh = ch as isize + dh[i];
        let nw = cw as isize + dw[i];
        if nh < 0 || nh >= grid.len() as isize || nw < 0 || nw >= grid[0].len() as isize {
            continue;
        }
        let nh = nh as usize;
        let nw = nw as usize;
        m = max(m, rec(nh, nw, grid));
    }
    grid[ch][cw] = true;
    m + 1
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3
1 1 0
1 0 1
1 1 0",
            "5"
        ),
        (
            r"5
3
1 1 1 0 1
1 1 0 0 0
1 0 0 0 1",
            "5"
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