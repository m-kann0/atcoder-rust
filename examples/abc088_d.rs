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

    let mut white_count: usize = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        white_count += line.iter().filter(|c| **c == '.').count();
        map.push(line);
    }

    const INF: usize = 100000;
    let di: Vec<isize> = vec![-1, 1, 0, 0];
    let dj: Vec<isize> = vec![0, 0, -1, 1];

    let mut d: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0, 0));
    d[0][0] = 1;
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        let ci = current.0;
        let cj = current.1;

        for direction in 0..4 {
            let ni = ci as isize + di[direction];
            let nj = cj as isize + dj[direction];
            if ni < 0 || nj < 0 || ni as usize >= h || nj as usize >= w {
                continue;
            }
            if map[ni as usize][nj as usize] == '#' || d[ni as usize][nj as usize] != INF {
                continue;
            }
            q.push_back((ni as usize, nj as usize));
            d[ni as usize][nj as usize] = d[ci][cj] + 1;
        }
    }

    if d[h - 1][w - 1] == INF {
        return "-1".to_string();
    }
    let ans = white_count - d[h - 1][w - 1];
    return ans.to_string();
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