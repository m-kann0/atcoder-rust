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
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for x in 0..h {
        for y in 0..w {
            if map[x][y] == 's' {
                start = (x, y);
            } else if map[x][y] == 'g' {
                goal = (x, y);
            }
        }
    }

    const INF: usize = 1_000_000_000;
    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut d: Vec<Vec<usize>> = vec![vec![INF; w]; h];

    q.push_back(start);
    d[start.0][start.1] = 0;

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
                if map[nx][ny] == '#' {
                    q.push_back((nx, ny));
                    d[nx][ny] = d[cx][cy] + 1;
                } else {
                    q.push_front((nx, ny));
                    d[nx][ny] = d[cx][cy];
                }
            }
        }
    }

    // eprintln!();
    // for line in &d {
    //     eprintln!("{:?}", line);
    // }

    if d[goal.0][goal.1] <= 2 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 5
s####
....#
#####
#...g",
            r"YES"
        ),
        (
            r"4 4
...s
....
....
.g..",
            r"YES"
        ),
        (
            r"10 10
s.........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
g##.#.#.#.
###.#.#.#.
###.#.#.#.
#.....#...",
            r"YES"
        ),
        (
            r"6 6
.....s
###...
###...
######
...###
g.####",
            r"YES"
        ),
        (
            r"1 10
s..####..g",
            r"NO"
        ),
        (
            r"1 10
s.##..##.g",
            r"NO"
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