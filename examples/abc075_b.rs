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

    let mut result = String::new();
    for ch in 0..h {
        for cw in 0..w {
            if grid[ch][cw] == '#' {
                result.push('#');
                continue;
            }
            let mut count = 0;
            for dh in -1..=1 {
                for dw in -1..=1 {
                    let nh = ch as isize + dh;
                    let nw = cw as isize + dw;
                    if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                        continue;
                    }
                    if grid[nh as usize][nw as usize] == '#' {
                        count += 1;
                    }
                }
            }
            result.push_str(&format!("{}", count.to_string()));
        }
        result.push('\n');
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 5
.....
.#.#.
.....",
            "11211
1#2#1
11211"
        ),
        (
            r"3 5
#####
#####
#####",
            "#####
#####
#####"
        ),
        (
            r"6 6
#####.
#.#.##
####.#
.#..#.
#.##..
#.#...",
            "#####3
#8#7##
####5#
4#65#2
#5##21
#4#310"
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