#![allow(non_snake_case)]

use std::io::Read;
use std::collections::{HashMap, VecDeque};

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

    let mut s = (0, 0);
    let mut g = (0, 0);
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' {
                s = (i, j);
            }
            if grid[i][j] == 'G' {
                g = (i, j);
            }
            if grid[i][j] == 'S' || grid[i][j] == 'G' || grid[i][j] == '.' || grid[i][j] == '#' {
                continue;
            }
            let c = grid[i][j];
            map.entry(c).or_insert(Vec::new()).push((i, j));
        }
    }

    let dh = vec![1, -1, 0, 0];
    let dw = vec![0, 0, 1, -1];

    const INF: usize = 1_000_000_000;
    let mut dist = vec![vec![INF; w]; h];
    let mut q = VecDeque::new();
    q.push_back(s);
    dist[s.0][s.1] = 0;
    while let Some((ch, cw)) = q.pop_front() {
        if (ch, cw) == g {
            break;
        }

        for i in 0..4 {
            let nh: isize = ch as isize + dh[i];
            let nw: isize = cw as isize + dw[i];
            if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                continue;
            }
            let nh = nh as usize;
            let nw = nw as usize;
            if grid[nh][nw] == '#' {
                continue;
            }
            if dist[nh][nw] != INF {
                continue;
            }
            dist[nh][nw] = dist[ch][cw] + 1;
            q.push_back((nh, nw));
        }
        let c = grid[ch][cw];
        if let Some(list) = map.get(&c) {
            for &(nh, nw) in list {
                if dist[nh][nw] != INF {
                    continue;
                }
                dist[nh][nw] = dist[ch][cw] + 1;
                q.push_back((nh, nw));
            }
        }
        map.remove(&c);
    }

    if dist[g.0][g.1] == INF {
        return "-1".to_string();
    }
    dist[g.0][g.1].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 5
S.b.b
a.a.G",
            "4"
        ),
        (
            r"11 11
S##...#c...
...#d.#.#..
..........#
.#....#...#
#.....bc...
#.##......#
.......c..#
..#........
a..........
d..#...a...
.#........G",
            "14"
        ),
        (
            r"11 11
.#.#.e#a...
.b..##..#..
#....#.#..#
.#dd..#..#.
....#...#e.
c#.#a....#.
.....#..#.e
.#....#b.#.
.#...#..#..
......#c#G.
#..S...#...",
            "-1"
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