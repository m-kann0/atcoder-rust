#![allow(non_snake_case)]

use std::io::Read;
use std::mem::swap;
use std::fmt::Display;

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
    let mut s = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        s.push(line);
    }
    let mut t = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        t.push(line);
    }

    let mut th = h as isize;
    let mut tw = w as isize;
    for _ in 0..4 {
        // eprintln!();
        // debug(&t);
        for i in (-th + 1)..h as isize {
            for j in (-tw + 1)..w as isize {
                let mut is_ok = true;
                for ti in 0..th {
                    for tj in 0..tw {
                        if t[ti as usize][tj as usize] != '#' {
                            continue;
                        }
                        let si = i + ti;
                        let sj = j + tj;
                        if si < 0 || si >= h as isize || sj < 0 || sj >= w as isize {
                            // eprintln!("(i, j, ti, tj) = {:?}", (i, j, ti, tj));
                            is_ok = false;
                            break;
                        }
                        if s[si as usize][sj as usize] == '#' {
                            // eprintln!("(i, j, ti, tj) = {:?}", (i, j, ti, tj));
                            is_ok = false;
                            break;
                        }
                    }
                    if !is_ok {
                        break;
                    }
                }
                if is_ok {
                    return "Yes".to_string();
                }
            }
        }

        t = grid::rotate_right(&t);
        swap(&mut th, &mut tw);
    }
    "No".to_string()
}

pub mod grid {
    pub fn rotate_right<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
        if grid.is_empty() {
            return Vec::new();
        }
        let h = grid.len();
        let w = grid[0].len();
        let mut result = Vec::with_capacity(w);
        for j in 0..w {
            let mut line = Vec::with_capacity(h);
            for i in (0..h).rev() {
                line.push(grid[i][j].clone());
            }
            result.push(line);
        }
        result
    }
}

fn debug<T: Display>(grid: &Vec<Vec<T>>) {
    for line in grid {
        for e in line {
            print!("{:3}", e);
        }
        println!();
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
...
.#.
..#
#.#
###
...",
            "Yes"
        ),
        (
            r"3 3
...
#..
#.#
.#.
.##
##.",
            "No"
        ),
        (
            r"2 5
.....
..#..
..##.
.###.",
            "Yes"
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