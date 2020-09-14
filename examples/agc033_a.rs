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

    const INF: usize = 1_000_000_000;
    let mut d: Vec<Vec<usize>> = vec![vec![INF; w]; h];

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '#' {
                d[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }

    let dh = vec![-1, 0, 1, 0];
    let dw = vec![0, -1, 0, 1];
    while let Some((i, j)) = q.pop_front() {
        for k in 0..4 {
            let ni: isize = i as isize + dh[k];
            let nj: isize = j as isize + dw[k];
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if d[ni][nj] != INF {
                continue;
            }
            d[ni][nj] = d[i][j] + 1;
            q.push_back((ni, nj));
        }
    }

    let mut ans = 0;
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
        (
            r"1 1
#",
            "0"
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