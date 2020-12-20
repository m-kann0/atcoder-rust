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
    let gh: usize = iterator.next().unwrap().parse().unwrap();
    let gw: usize = iterator.next().unwrap().parse().unwrap();
    let gh = gh - 1;
    let gw = gw - 1;
    let mut grid = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        grid.push(line);
    }

    let dh = vec![1, -1, 0, 0];
    let dw = vec![0, 0, 1, -1];
    let c = vec!['^', 'v', '<', '>'];

    // let mut result = vec![vec![false; w]; h];

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];
    q.push_back((gh, gw));
    visited[gh][gw] = true;
    while let Some((ch, cw)) = q.pop_front() {
        for i in 0..4 {
            let nh = ch as isize + dh[i];
            let nw = cw as isize + dw[i];
            if nh < 0 || nh >= h as isize || nw < 0 || nw >= w as isize {
                continue;
            }
            if grid[nh as usize][nw as usize] == '#' || visited[nh as usize][nw as usize] {
                continue;
            }
            if grid[nh as usize][nw as usize] == '.' || grid[nh as usize][nw as usize] == c[i] {
                q.push_back((nh as usize, nw as usize));
                visited[nh as usize][nw as usize] = true;
            }
        }
    }

    let mut output = String::new();
    for i in 0..h {
        for j in 0..w {
            let o =
                if grid[i][j] == '#' {
                    '#'
                } else if visited[i][j] {
                    'o'
                } else {
                    'x'
                };
            output.push(o);
        }
        output.push('\n');
    }
    output
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
1 1
..#
^^.
><.",
            "oo#
ooo
xxo"
        ),
        (
            r"10 12
9 1
#.^<..><<...
#<>.#<^.<<.^
^.<>.^.^.^>.
^.>#^><#....
.>.^>#...<<>
....^^.#<.<.
.>^..^#><#.^
......#>....
..<#<...^>^.
<..^>^^...^<",
            "#xxxxxxxxxxx
#xxx#xxxxxxx
xooxxxxxxxxx
xox#xxx#xxxx
oooxx#xxxxxx
ooooxxx#xxxx
ooooox#xx#xx
oooooo#xxxxx
ooo#xoooxxxx
xooxooooooxx"
        ),
        (
            r"15 20
13 9
####..<#^>#>.<<><^..
#.>#>.^#^.>><>...^..
>..<>.#.>.>.>...#..<
<^>.#..<>^#<#.>.<.^.
>#<^>.>#^>#^.^.#^><^
<^.^.#<...<.><#>...#
.<>....^..#>>#..>>><
.<#<^#.>#>^^.>.##.^<
.#.^.....<<#^#><^<<<
^.#>.#^.>.^.^<<>..><
.^#^<^^^<......^>.#^
.<..#>...^>^.^<..<.^
#.^.#..#.....>#.^^.>
.#^..>>><>>>^..#^.^^
.>#..<..<>.#>..^.#.^",
            "####xxx#xx#xxxxxxxxx
#xx#xxx#xxxxxxxxxxxx
xxxxxx#xxxxxxxxx#xxx
xxxx#xxxxx#x#xxxxxxx
x#xxxxx#xx#xxxx#xxxx
xxoxo#xxxxxxxx#xxxx#
xxoooooxxx#xx#xxxxxx
xx#xo#ox#xxxxxx##xxx
x#xxooooooo#x#xxxxxx
xx#oo#ooooooxxxoooxx
xx#ooxoooooooooooo#x
xxoo#oooooooooooooox
#ooo#oo#ooooox#oooox
x#oooxxxxoooooo#ooox
xx#oooooooo#oooxo#ox"
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
            println!("    Expected:\n{}", expected);
            println!("    Actual  :\n{}", actual);

            all_ok = false;
        }
    }

    assert_eq!(all_ok, true);
}