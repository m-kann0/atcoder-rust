use std::io::Read;
use std::collections::{VecDeque, HashSet};

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
    let ch: usize = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
    let cw: usize = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
    let dh: usize = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
    let dw: usize = iterator.next().unwrap().parse::<usize>().unwrap() - 1;
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    const INF: usize = 1_000_000_000;
    let mut colors = vec![vec![INF; w]; h];
    let mut color = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '#' {
                continue;
            }
            if colors[i][j] != INF {
                continue;
            }

            let di = vec![-1, 1, 0, 0];
            let dj = vec![0, 0, -1, 1];

            let mut q: VecDeque<(usize, usize)> = VecDeque::new();
            colors[i][j] = color;
            q.push_back((i, j));
            while let Some((ci, cj)) = q.pop_front() {
                for k in 0..4 {
                    let ni = ci as isize + di[k];
                    let nj = cj as isize + dj[k];
                    if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if map[ni][nj] == '#' {
                        continue;
                    }
                    if colors[ni][nj] != INF {
                        continue;
                    }
                    colors[ni][nj] = color;
                    q.push_back((ni, nj));
                }
            }
            color += 1;
        }
    }

    if colors[ch][cw] == colors[dh][dw] {
        return "0".to_string();
    }

    // let mut graph: Vec<Vec<bool>> = vec![vec![false; color]; color];
    let mut graph2: Vec<HashSet<usize>> = vec![HashSet::new(); color];
    for i in 0..h {
        for j in 0..w {
            if colors[i][j] == INF {
                continue;
            }
            for di in -2..=2 {
                for dj in -2..=2 {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if colors[ni][nj] == INF {
                        continue;
                    }
                    if colors[i][j] != colors[ni][nj] {
                        graph2[colors[i][j]].insert(colors[ni][nj]);
                        graph2[colors[ni][nj]].insert(colors[i][j]);
                        // graph[colors[i][j]][colors[ni][nj]] = true;
                        // graph[colors[ni][nj]][colors[i][j]] = true;
                    }
                }
            }
        }
    }

    // let mut graph2: Vec<Vec<usize>> = vec![Vec::new(); color];
    // for i in 0..color {
    //     for j in 0..color {
    //         if graph[i][j] {
    //             graph2[i].push(j);
    //         }
    //     }
    // }

    let mut d: Vec<usize> = vec![INF; color];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(colors[ch][cw]);
    d[colors[ch][cw]] = 0;
    while let Some(c) = q.pop_front() {
        for &n in &graph2[c] {
            if d[n] != INF {
                continue;
            }
            d[n] = d[c] + 1;
            q.push_back(n);
        }
    }
    if d[colors[dh][dw]] == INF {
        "-1".to_string()
    } else {
        d[colors[dh][dw]].to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4
1 1
4 4
..#.
..#.
.#..
.#..",
            "1"
        ),
        (
            r"4 4
1 4
4 1
.##.
####
####
.##.",
            "-1"
        ),
        (
            r"4 4
2 2
3 3
....
....
....
....",
            "0"
        ),
        (
            r"4 5
1 2
2 5
#.###
####.
#..##
#..##",
            "2"
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