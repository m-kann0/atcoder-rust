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

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut grid = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        grid.push(line);
    }
    let mut p = vec![(0, 0); n + 1];
    for i in 0..h {
        for j in 0..w {
            match grid[i][j] {
                '.' | 'X' => { continue },
                'S' => { p[0] = (i, j); },
                _ => { p[grid[i][j].to_digit(10).unwrap() as usize] = (i, j); },
            }
        }
    }

    const INF: usize = 1_000_000_000;
    let dh = vec![1, -1, 0, 0];
    let dw = vec![0, 0, 1, -1];

    let mut ans = 0;
    for i in 0..n {
        let (sh, sw) = p[i];
        let (gh, gw) = p[i + 1];

        let mut dist = vec![vec![INF; w]; h];
        let mut q = VecDeque::new();
        dist[sh][sw] = 0;
        q.push_back((sh, sw));
        while let Some((ch, cw)) = q.pop_front() {
            for i in 0..4 {
                let nh = ch as isize + dh[i];
                let nw = cw as isize + dw[i];
                if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                    continue;
                }
                let nh = nh as usize;
                let nw = nw as usize;
                if grid[nh][nw] == 'X' || dist[nh][nw] != INF {
                    continue;
                }
                dist[nh][nw] = dist[ch][cw] + 1;
                q.push_back((nh, nw));
            }
        }
        ans += dist[gh][gw];
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 1
S..
...
..1",
            "4"
        ),
        (
            r"4 5 2
.X..1
....X
.XX.S
.2.X.",
            "12"
        ),
        (
            r"10 10 9
.X...X.S.X
6..5X..X1X
...XXXX..X
X..9X...X.
8.X2X..X3X
...XX.X4..
XX....7X..
X..X..XX..
X...X.XX..
..X.......",
            "91"
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