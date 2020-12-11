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
    let mut grid = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        grid.push(line);
    }

    let mut white: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '.' {
                white += 1;
            }
        }
    }

    const INF: usize = 1_000_000;
    let dh = vec![1, -1, 0, 0];
    let dw = vec![0, 0, 1, -1];
    let mut dist = vec![vec![INF; w]; h];
    let mut q = VecDeque::new();
    dist[0][0] = 1;
    q.push_back((0, 0));
    while let Some((ch, cw)) = q.pop_front() {
        for i in 0..4 {
            let nh = ch as isize + dh[i];
            let nw = cw as isize + dw[i];
            if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                continue;
            }
            let nh = nh as usize;
            let nw = nw as usize;
            if grid[nh][nw] == '.' && dist[nh][nw] == INF {
                dist[nh][nw] = dist[ch][cw] + 1;
                q.push_back((nh, nw));
            }
        }
    }
    if dist[h - 1][w - 1] == INF {
        return "-1".to_string();
    }
    (white - dist[h - 1][w - 1]).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
..#
#..
...",
            "2"
        ),
        (
            r"10 37
.....................................
...#...####...####..###...###...###..
..#.#..#...#.##....#...#.#...#.#...#.
..#.#..#...#.#.....#...#.#...#.#...#.
.#...#.#..##.#.....#...#.#.###.#.###.
.#####.####..#.....#...#..##....##...
.#...#.#...#.#.....#...#.#...#.#...#.
.#...#.#...#.##....#...#.#...#.#...#.
.#...#.####...####..###...###...###..
.....................................",
            "209"
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