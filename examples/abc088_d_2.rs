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

    const INF: usize = 10_000;
    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut d: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    q.push_back((0, 0));
    d[0][0] = 1;
    while let Some((cx, cy)) = q.pop_front() {
        if cx == h - 1 && cy == w - 1 {
            break;
        }
        for i in 0..4 {
            let nx = cx as isize + dx[i];
            let ny = cy as isize + dy[i];
            if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if map[nx][ny] != '#' && d[nx][ny] == INF {
                q.push_back((nx, ny));
                d[nx][ny] = d[cx][cy] + 1;
            }
        }
    }

    if d[h - 1][w - 1] == INF {
        return "-1".to_string();
    }

    let mut white: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '.' {
                white += 1;
            }
        }
    }

    let ans = white - d[h - 1][w - 1];
    ans.to_string()
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