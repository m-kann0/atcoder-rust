use std::io::Read;
use std::collections::VecDeque;
use std::cmp::max;

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
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    const INF: usize = 1_000_005;
    let mut d: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '#' {
                q.push_back((i, j));
                d[i][j] = 0;
            }
        }
    }
    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    while let Some((cx, cy)) = q.pop_front() {
        for i in 0..4 {
            let nx = cx as isize + dx[i];
            let ny = cy as isize + dy[i];
            if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if d[nx][ny] == INF {
                q.push_back((nx, ny));
                d[nx][ny] = d[cx][cy] + 1;
            }
        }
    }

    let mut ans: usize = 0;
    for i in 0..h {
        for j in 0..w {
            ans = max(ans, d[i][j]);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
...
.#.
...",
            "2"
        ),
        (
            r"6 6
..#..#
......
#..#..
......
.#....
....#.",
            "3"
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