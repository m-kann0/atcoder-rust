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
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        grid.push(line);
    }

    let mut len = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                len += 1;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != '#' {
                continue;
            }

            let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
            let mut result: Vec<(usize, usize)> = Vec::new();
            rec((i, j), (INF, INF), 1, &mut result, h, w, &grid, len, &mut visited);
            if result.len() == len {
                let mut output = String::new();
                output.push_str(&format!("{}\n", len));
                for &(ri, rj) in result.iter() {
                    output.push_str(&format!("{} {}\n", ri + 1, rj + 1));
                }
                return output
            }
        }
    }

    "0".to_string()
}

fn rec(
    current: (usize, usize), from: (usize, usize), d: usize, result: &mut Vec<(usize, usize)>,
    h: usize, w: usize, grid: &Vec<Vec<char>>, len: usize, visited: &mut Vec<Vec<bool>>
) {
    if d == len {
        result.push(current);
        return;
    }

    let (ch, cw) = current;
    visited[ch][cw] = true;

    let dh = vec![0, -1, 0, 1];
    let dw = vec![1, 0, -1, 0];
    for i in 0..4 {
        let nh = ch as isize + dh[i];
        let nw = cw as isize + dw[i];
        if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
            continue;
        }
        let nh = nh as usize;
        let nw = nw as usize;
        if grid[nh][nw] != '#' || visited[nh][nw] {
            continue;
        }
        if (nh, nw) == from {
            continue;
        }
        rec((nh, nw), (ch, cw), d + 1, result, h, w, grid, len, visited);
        if !result.is_empty() {
            result.push((ch, cw));
            return;
        }
    }
    visited[ch][cw] = false;
}

const INF: usize = 10;



#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
##.
.##
###",
            "7
1 1
1 2
2 2
2 3
3 3
3 2
3 1"
        ),
        (
            r"3 4
####
####
.#..",
            "9
1 4
2 4
2 3
1 3
1 2
1 1
2 1
2 2
3 2"
        ),
        (
            r"3 3
.##
###
###",
            "8
1 2
1 3
2 3
2 2
2 1
3 1
3 2
3 3"
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